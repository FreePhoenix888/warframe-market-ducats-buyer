#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // It's an example

mod external_lib;
mod lib;

use eframe::egui;
use eframe::egui::{Align, Button, DragValue, Frame, Layout, Rounding, ScrollArea, Spinner, Stroke, TextEdit};
use std::sync::mpsc::{self, TryRecvError};

fn main() -> eframe::Result {
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
    rx: mpsc::Receiver<Result<Vec<lib::Order>, String>>,
    tx: mpsc::Sender<Result<Vec<lib::Order>, String>>,
    orders: Option<Vec<lib::Order>>,
    loading: bool,
    user_inputs: UserInputs,
    default_inputs: UserInputs,
    error_message: Option<String>,
}

impl Default for UserInputs {
    fn default() -> Self {
        // Replace these with actual default values from your lib
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
        let (tx, rx) = mpsc::channel();
        let default_inputs = UserInputs::default();
        Self {
            rx,
            tx,
            orders: None,
            loading: false,
            user_inputs: default_inputs.clone(),
            default_inputs,
            error_message: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll the channel for new messages without blocking the UI.
        match self.rx.try_recv() {
            Ok(result) => {
                match result {
                    Ok(data) => {
                        self.orders = Some(data);
                        self.error_message = None;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.orders = None;
                    }
                }
                self.loading = false;
            }
            Err(TryRecvError::Empty) => {
                /* No new data */
            }
            Err(TryRecvError::Disconnected) => {
                self.loading = false;
            }
        }

        let max_price = self.user_inputs.max_price_to_search.parse::<i32>().unwrap_or_default();
        let min_quantity = self.user_inputs.min_quantity_to_search.parse::<i32>().unwrap_or_default();
        let offer_price = self.user_inputs.price_to_offer.parse::<i32>().unwrap_or_default();
        let item_names: Vec<String> = self.user_inputs.item_names
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.heading("My egui Application with Async Orders");
                ui.add_space(20.0);

                // Input fields section
                egui::Grid::new("input_grid").num_columns(3).show(ui, |ui| {
                    ui.label("Max Price:");
                    if let Ok(mut value) = self.user_inputs.max_price_to_search.parse::<i32>() {
                        if ui.add(DragValue::new(&mut value)
                            .clamp_range(0..=i32::MAX)
                            .speed(1)
                        ).changed() {
                            self.user_inputs.max_price_to_search = value.to_string();
                        }
                    }
                    ui.end_row();

                    ui.label("Min Quantity:");
                    if let Ok(mut value) = self.user_inputs.min_quantity_to_search.parse::<i32>() {
                        if ui.add(DragValue::new(&mut value)
                            .clamp_range(0..=i32::MAX)
                            .speed(1)
                        ).changed() {
                            self.user_inputs.min_quantity_to_search = value.to_string();
                        }
                    }
                    ui.end_row();

                    ui.label("Offer Price:");
                    if let Ok(mut value) = self.user_inputs.price_to_offer.parse::<i32>() {
                        if ui.add(DragValue::new(&mut value)
                            .clamp_range(0..=i32::MAX)
                            .speed(1)
                        ).changed() {
                            self.user_inputs.price_to_offer = value.to_string();
                        }
                    }
                    ui.end_row();
                });;

                ui.add_space(10.0);

                ui.label("Item Names (one per line):");
                ui.add(TextEdit::multiline(&mut self.user_inputs.item_names)
                    .hint_text("Enter item names (one per line)")
                    .desired_width(f32::INFINITY)
                    .min_size([ui.available_width(), 100.0].into()));

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Reset to Defaults").clicked() {
                        self.user_inputs = self.default_inputs.clone();
                    }

                    if ui.add_sized([150.0, 30.0], Button::new("Fetch Orders")).clicked() && !self.loading {
                        self.loading = true;
                        let tx = self.tx.clone();



                        std::thread::spawn(move || {
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            let result = rt.block_on(async {

                                let min_quantity = min_quantity;
                                let max_price = max_price;

                                let filter_orders = |order: &lib::Order| -> bool {
                                    return order.user.status == "ingame" &&
                                        order.visible &&
                                        order.order_type == "sell" &&
                                        order.platinum <= max_price &&
                                        order.quantity >= min_quantity;
                                };

                                match lib::fetch_all_orders(&item_names).await {
                                    Ok(orders) => {
                                        let filtered_orders = lib::filter_orders(orders, filter_orders);
                                        let processed_orders = lib::process_orders(filtered_orders);
                                        Ok(processed_orders)
                                    },
                                    Err(e) => Err(format!("{:?}", e)),
                                }
                            });
                            let _ = tx.send(result);
                        });
                    }                });

                ui.add_space(20.0);

                if self.loading {
                    ui.add(Spinner::new().size(32.0));
                }

                if let Some(orders) = &self.orders {
                    ui.label("Fetched Order Messages:");
                    ui.add_space(10.0);

                    ScrollArea::new(true).show(ui, |ui| {
                        for (i, order) in orders.iter().enumerate() {
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
                                    let button = ui.add_sized([100.0, 100.0], Button::new(message.clone()));
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

        ctx.request_repaint();
    }
}