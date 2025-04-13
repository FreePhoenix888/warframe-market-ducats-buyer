#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // It's an example

mod external_lib;
mod lib;

use eframe::egui;
use eframe::egui::{
    Align, Button, DragValue, Frame, Layout, Rounding, ScrollArea, Spinner, Stroke, TextEdit,
};
use std::sync::mpsc::{self, TryRecvError};
use log::{debug, error, info, warn};

fn main() -> eframe::Result {
    if std::env::var("RUST_LOG").is_err() {
        // Set a default log level if RUST_LOG is not set
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init(); // Log to stderr (use RUST_LOG=debug for details)
    let options = eframe::NativeOptions {
        window_builder: Some(Box::new(|builder| builder.with_maximized(true))),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App with Async Orders",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Clone)]
struct UserInputs {
    max_price_to_search: String,
    min_quantity_to_search: String,
    price_to_offer: String,
    item_names: String,
}

struct MyApp {
    rx_fetch: mpsc::Receiver<Result<Vec<lib::Order>, String>>,
    tx_fetch: mpsc::Sender<Result<Vec<lib::Order>, String>>,
    rx_process: mpsc::Receiver<Result<Vec<lib::Order>, String>>,
    tx_process: mpsc::Sender<Result<Vec<lib::Order>, String>>,
    orders: Option<Vec<lib::Order>>,
    processed_orders: Option<Vec<lib::Order>>,
    loading_fetch: bool,
    loading_process: bool,
    user_inputs: UserInputs,
    default_inputs: UserInputs,
    error_message: Option<String>,
    show_settings: bool, // New field to toggle settings visibility
}

impl Default for UserInputs {
    fn default() -> Self {
        Self {
            max_price_to_search: lib::MAX_PRICE_TO_SEARCH.to_string(),
            min_quantity_to_search: lib::MIN_QUANTITY_TO_SEARCH.to_string(),
            price_to_offer: lib::PRICE_TO_OFFER.to_string(),
            item_names: lib::PROFITABLE_ITEM_NAMES.join("\n").to_string(),
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx_fetch, rx_fetch) = mpsc::channel();
        let (tx_process, rx_process) = mpsc::channel();
        let default_inputs = UserInputs::default();
        Self {
            rx_fetch,
            tx_fetch,
            rx_process,
            tx_process,
            orders: None,
            processed_orders: None,
            loading_fetch: false,
            loading_process: false,
            user_inputs: default_inputs.clone(),
            default_inputs,
            error_message: None,
            show_settings: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll the fetch channel for new messages
        match self.rx_fetch.try_recv() {
            Ok(result) => {
                match result {
                    Ok(data) => {
                        info!("Successfully received fetched data. len {:?}", data.len());
                        self.orders = Some(data);
                        self.error_message = None;
                    }
                    Err(err) => {
                        error!("Error fetching orders: {}", err);
                        self.error_message = Some(err);
                        self.orders = None;
                    }
                }
                self.loading_fetch = false;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                warn!("Fetch channel disconnected.");
                self.loading_fetch = false;
            }
        }

        // Poll the process channel for new messages
        match self.rx_process.try_recv() {
            Ok(result) => {
                match result {
                    Ok(data) => {
                        self.processed_orders = Some(data);
                        self.error_message = None;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.processed_orders = None;
                    }
                }
                self.loading_process = false;
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.loading_process = false;
            }
        }

        let max_price = self
            .user_inputs
            .max_price_to_search
            .parse::<u32>()
            .unwrap_or_default();
        let min_quantity = self
            .user_inputs
            .min_quantity_to_search
            .parse::<u32>()
            .unwrap_or_default();
        let offer_price = self
            .user_inputs
            .price_to_offer
            .parse::<u32>()
            .unwrap_or_default();
        let item_names: Vec<String> = self
            .user_inputs
            .item_names
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Warframe Market Ducats Buyer");
                    ui.add_space(20.0);

                    // Attribution and Credit
                    ui.label("This app works thanks to ");
                    ui.hyperlink("https://warframe.market/");
                    ui.label("Made by ");
                    ui.hyperlink_to("FreePhoenix888", "https://github.com/FreePhoenix888");
                    ui.hyperlink_to("Source Code", "https://github.com/FreePhoenix888/warframe-market-ducats-buyer");

                    ui.add_space(20.0);

                    if ui.button("Settings").clicked() {
                        self.show_settings = !self.show_settings;
                    }

                    ui.add_space(20.0);

                    let is_enabled_button_fetch_orders = !self.loading_fetch;
                    ui.add_enabled_ui(
                        is_enabled_button_fetch_orders,
                        |ui| {
                            if ui
                                .add_sized([150.0, 30.0], Button::new("Fetch Orders"))
                                .clicked()
                            {
                                info!("Starting to fetch orders...");
                                self.loading_fetch = true;
                                let tx = self.tx_fetch.clone();

                                std::thread::spawn(move || {
                                    let rt = tokio::runtime::Runtime::new().unwrap();
                                    let result = rt.block_on(async {
                                        match lib::fetch_all_orders(&item_names).await {
                                            Ok(orders) => {
                                                info!("Successfully fetched orders.");
                                                Ok(orders)
                                            },
                                            Err(e) => {
                                                error!("Failed to fetch orders: {:?}", e);
                                                Err(format!("{:?}", e))
                                            },
                                        }
                                    });
                                    let _ = tx.send(result);
                                });
                            }
                        }
                    );

                    let orders_len = self.orders.as_ref().map_or(0, |orders| orders.len());
                    ui.label(format!("Orders length: {}", orders_len));


                    let is_enabled_button_process_orders = !self.loading_process
                        && orders_len > 0;
                    ui.add_enabled_ui(
                        is_enabled_button_process_orders,
                        |ui| {
                            if ui
                                .add_sized([150.0, 30.0], Button::new("Filter & Process Orders"))
                                .clicked()
                                {
                                    self.loading_process = true;
                                    let tx = self.tx_process.clone();
                                    let orders = self.orders.clone();

                                    std::thread::spawn(move || {
                                        let filter_orders = |order: &lib::Order| -> bool {
                                            order.user.status == "ingame"
                                                && order.visible
                                                && order.order_type == "sell"
                                                && order.platinum <= max_price
                                                && order.quantity >= min_quantity
                                        };

                                        let processed_orders = orders
                                            .map(|o| lib::process_orders(o, filter_orders))
                                            .unwrap_or_else(Vec::new);
                                        let _ = tx.send(Ok(processed_orders));
                                    });
                                }

                        }
                    );
                });

                let processed_orders_len = self.processed_orders.as_ref().map_or(0, |orders| orders.len());
                ui.label(format!("Processed orders length: {}", processed_orders_len));

                ui.add_space(20.0);

                if self.loading_fetch {
                    ui.add(Spinner::new().size(32.0));
                }

                // if let Some(orders) = &self.orders {
                //     ui.label("Fetched Orders:");
                //     ui.add_space(10.0);
                //
                //     ScrollArea::new(true).show(ui, |ui| {
                //         for order in orders {
                //             ui.label(format!("{:?}", order));
                //         }
                //     });
                // }
                //
                // if self.loading_process {
                //     ui.add(Spinner::new().size(32.0));
                // }

                if let Some(processed_orders) = &self.processed_orders {
                    ui.label("Processed Orders:");
                    ui.add_space(10.0);

                    ScrollArea::new(true).show(ui, |ui| {
                        for (i, order) in processed_orders.iter().enumerate() {
                            let frame_stroke = if order.is_with_group.unwrap_or(false) {
                                Stroke::new(2.0, ui.visuals().selection.stroke.color) // Highlighted outline
                            } else {
                                Stroke::new(1.0, ui.visuals().extreme_bg_color) // Default outline
                            };

                            let message = lib::generate_message(order, offer_price);

                            Frame::none()
                                .stroke(frame_stroke)
                                .rounding(Rounding::same(5)) // Optional: rounded corners
                                .show(ui, |ui| {
                                    let button =
                                        ui.add_sized([100.0, 100.0], Button::new(message.clone()));
                                    if button.clicked() {
                                        ui.ctx().copy_text(message.clone());
                                    }
                                });

                            ui.add_space(8.0);
                        }
                    });
                }

                if let Some(error_message) = &self.error_message {
                    ui.colored_label(ui.visuals().warn_fg_color, error_message);
                }
            });
        });

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("Max Price:");
                    if let Ok(mut value) = self.user_inputs.max_price_to_search.parse::<u32>() {
                        if ui
                            .add(
                                DragValue::new(&mut value)
                                    .clamp_range(0..=10)
                                    .speed(0.02),
                            )
                            .changed()
                        {
                            self.user_inputs.max_price_to_search = value.to_string();
                        }
                    }
                    ui.end_row();

                    ui.label("Min Quantity:");
                    if let Ok(mut value) = self.user_inputs.min_quantity_to_search.parse::<u32>() {
                        if ui
                            .add(
                                DragValue::new(&mut value)
                                    .clamp_range(0..=10)
                                    .speed(0.02),
                            )
                            .changed()
                        {
                            self.user_inputs.min_quantity_to_search = value.to_string();
                        }
                    }
                    ui.end_row();

                    ui.label("Offer Price:");
                    if let Ok(mut value) = self.user_inputs.price_to_offer.parse::<u32>() {
                        if ui
                            .add(
                                DragValue::new(&mut value)
                                    .clamp_range(0..=10)
                                    .speed(0.02),
                            )
                            .changed()
                        {
                            self.user_inputs.price_to_offer = value.to_string();
                        }
                    }
                    ui.end_row();

                    ui.add_space(10.0);

                    ui.label("Item Names (one per line):");
                    ui.add(
                        TextEdit::multiline(&mut self.user_inputs.item_names)
                            .hint_text("Enter item names (one per line)")
                            .desired_width(f32::INFINITY)
                            .min_size([ui.available_width(), 100.0].into()),
                    );

                    ui.add_space(10.0);

                    if ui.button("Reset to Defaults").clicked() {
                        self.user_inputs = self.default_inputs.clone();
                    }
                });
        }

        ctx.request_repaint();
    }
}