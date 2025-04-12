#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // It's an example

mod external_lib;
mod lib;

use eframe::egui;
use eframe::egui::{Align, Button, Layout, ScrollArea, Spinner};
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

struct MyApp {
    rx: mpsc::Receiver<Vec<lib::Order>>,
    tx: mpsc::Sender<Vec<lib::Order>>,
    orders: Option<Vec<lib::Order>>,
    loading: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            rx,
            tx,
            orders: None,
            loading: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll the channel for new messages without blocking the UI.
        match self.rx.try_recv() {
            Ok(data) => {
                // Append the new message to the existing messages array.
                if let Some(messages) = &mut self.orders {
                    self.orders = Some(data);
                } else {
                    self.orders = Some(data);
                }
                self.loading = false;
            }
            Err(TryRecvError::Empty) => {
                /* No new data */
            }
            Err(TryRecvError::Disconnected) => {
                // TODO: what to do?
                self.loading = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                ui.heading("My egui Application with Async Orders");
                ui.add_space(20.0);

                let button = ui.add_sized([150.0, 100.0], Button::new("Fetch Orders"));

                if button.clicked() && !self.loading {
                    self.loading = true;
                    let tx = self.tx.clone();
                    std::thread::spawn(move || {
                        // Create a short-lived Tokio runtime.
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let result = rt.block_on(async {
                            // Fetch profitable orders.
                            lib::fetch_all_profitable_orders()
                                .await
                                .unwrap_or_else(|_| vec![])
                        });
                        // Send the generated messages back to the UI thread.
                        let _ = tx.send(result);
                    });
                }

                ui.add_space(20.0);

                if self.loading {
                    ui.add(Spinner::new().size(32.0));
                }

                if let Some(orders) = &self.orders {
                    ui.label("Fetched Order Messages:");
                    ui.add_space(10.0);

                    ScrollArea::new(true).show(ui, |ui| {
                        for (i, order) in orders.iter().enumerate() {
                            let bg_color = if order.is_with_group.unwrap_or(false) {
                                ui.visuals().selection.bg_fill
                            } else {
                                ui.visuals().extreme_bg_color
                            };

                            let message = lib::generate_message(order);

                            // Use a frame to style each message as a card

                            let button = ui.add_sized([100.0, 100.0], Button::new(message.clone()));
                            if button.clicked() {
                                ui.ctx().copy_text(message.clone());
                            }

                            ui.add_space(8.0);
                        }
                    });
                }
            });
        });

        // Ensure the UI updates continuously.
        ctx.request_repaint();
    }
}
