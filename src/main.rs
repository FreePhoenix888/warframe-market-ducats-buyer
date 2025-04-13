#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // It's an example

mod external_lib;
mod lib;

use eframe::egui;
use eframe::egui::{
  Align, Button, DragValue, Frame, Layout, Rounding, ScrollArea, Spinner,
  Stroke, TextEdit,
};
use egui_notify::Toasts;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, TryRecvError};

#[derive(Clone, Serialize, Deserialize)]
pub struct Preset {
  name: String,
  max_price_to_search: String,
  min_quantity_to_search: String,
  price_to_offer: String,
  item_names: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserInputs {
  max_price_to_search: String,
  min_quantity_to_search: String,
  price_to_offer: String,
  item_names: String,
  presets: Vec<Preset>,
  current_preset_name: Option<String>,
}

impl UserInputs {
  const SETTINGS_FILE: &'static str = "settings.json";

  pub fn load() -> Self {
    if let Ok(file) = std::fs::File::open(Self::SETTINGS_FILE) {
      if let Ok(settings) = serde_json::from_reader(file) {
        return settings;
      }
    }
    Self::default()
  }

  pub fn save(&self) {
    if let Ok(file) = std::fs::File::create(Self::SETTINGS_FILE) {
      let _ = serde_json::to_writer_pretty(file, self);
    }
  }

  pub fn save_as_preset(&mut self, name: String) {
    let preset = Preset {
      name: name.clone(),
      max_price_to_search: self.max_price_to_search.clone(),
      min_quantity_to_search: self.min_quantity_to_search.clone(),
      price_to_offer: self.price_to_offer.clone(),
      item_names: self.item_names.clone(),
    };

    // Remove existing preset with the same name if it exists
    self.presets.retain(|p| p.name != name);
    self.presets.push(preset);
    self.current_preset_name = Some(name);
    self.save();
  }

  pub fn load_preset(&mut self, name: &str) -> bool {
    if let Some(preset) = self.presets.iter().find(|p| p.name == name) {
      self.max_price_to_search = preset.max_price_to_search.clone();
      self.min_quantity_to_search = preset.min_quantity_to_search.clone();
      self.price_to_offer = preset.price_to_offer.clone();
      self.item_names = preset.item_names.clone();
      self.current_preset_name = Some(name.to_string());
      true
    } else {
      false
    }
  }

  pub fn delete_preset(&mut self, name: &str) {
    self.presets.retain(|p| p.name != name);
    if self.current_preset_name.as_deref() == Some(name) {
      self.current_preset_name = None;
    }
    self.save();
  }
}

impl Default for UserInputs {
  fn default() -> Self {
    Self {
      max_price_to_search: lib::MAX_PRICE_TO_SEARCH.to_string(),
      min_quantity_to_search: lib::MIN_QUANTITY_TO_SEARCH.to_string(),
      price_to_offer: lib::PRICE_TO_OFFER.to_string(),
      item_names: lib::PROFITABLE_ITEM_NAMES.join("\n").to_string(),
      presets: Vec::new(),
      current_preset_name: None,
    }
  }
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
  toasts: Toasts,
  show_settings: bool,
  show_credits: bool,
  show_all_orders: bool,
  new_preset_name: String,
}

impl Default for MyApp {
  fn default() -> Self {
    let (tx_fetch, rx_fetch) = mpsc::channel();
    let (tx_process, rx_process) = mpsc::channel();
    let user_inputs = UserInputs::load();
    Self {
      rx_fetch,
      tx_fetch,
      rx_process,
      tx_process,
      orders: None,
      processed_orders: None,
      loading_fetch: false,
      loading_process: false,
      user_inputs,
      toasts: Toasts::new(),
      show_settings: false,
      show_credits: false,
      show_all_orders: false,
      new_preset_name: String::new(),
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
            let message = format!("Successfully received fetched {:?} orders", data.len());
            info!("{}", message);
            self.orders = Some(data);
            self.toasts.success(message);
          }
          Err(err) => {
            error!("Error fetching orders: {}", err);
            self.toasts.error(format!("Error fetching orders: {}", err));
            self.orders = None;
          }
        }
        self.loading_fetch = false;
      }
      Err(TryRecvError::Empty) => {}
      Err(TryRecvError::Disconnected) => {
        warn!("Fetch channel disconnected.");
        self.loading_fetch = false;
        self.toasts.warning("Fetch channel disconnected.");
      }
    }

    // Poll the process channel for new messages
    match self.rx_process.try_recv() {
      Ok(result) => {
        match result {
          Ok(data) => {
            self.processed_orders = Some(data);
            self.toasts.success("Successfully processed orders.");
          }
          Err(err) => {
            self.toasts.error(format!("Error processing orders: {}", err));
            self.processed_orders = None;
          }
        }
        self.loading_process = false;
      }
      Err(TryRecvError::Empty) => {}
      Err(TryRecvError::Disconnected) => {
        self.loading_process = false;
        self.toasts.warning("Process channel disconnected.");
      }
    }

    let max_price = self.user_inputs.max_price_to_search.parse::<u32>().unwrap_or_default();
    let min_quantity = self.user_inputs.min_quantity_to_search.parse::<u32>().unwrap_or_default();
    let offer_price = self.user_inputs.price_to_offer.parse::<u32>().unwrap_or_default();
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
          #[cfg(feature = "debug_ui")]
          {
            if ui.button("Test Toast Notifications").clicked() {
              self.toasts.success("Successfully fetched orders.");
            }
          }

          ui.heading("Warframe Market Ducats Buyer");
          ui.add_space(20.0);

          if ui.button("Credits").clicked() {
            self.show_credits = !self.show_credits;
          }

          ui.add_space(20.0);

          if ui.button("Settings").clicked() {
            self.show_settings = !self.show_settings;
          }

          ui.add_space(20.0);

          let is_enabled_button_fetch_orders = !self.loading_fetch;
          ui.add_enabled_ui(is_enabled_button_fetch_orders, |ui| {
            if ui
                .add_sized([150.0, 30.0], Button::new("Fetch Orders"))
                .clicked()
            {
              info!("Starting to fetch orders...");
              self.loading_fetch = true;
              let tx = self.tx_fetch.clone();
              let item_names = item_names.clone();

              std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(async {
                  match lib::fetch_all_orders(&item_names).await {
                    Ok(orders) => {
                      info!("Successfully fetched orders.");
                      Ok(orders)
                    }
                    Err(e) => {
                      error!("Failed to fetch orders: {:?}", e);
                      Err(format!("{:?}", e))
                    }
                  }
                });
                let _ = tx.send(result);
              });
            }
          });

          let orders_len = self.orders.as_ref().map_or(0, |orders| orders.len());
          ui.label(format!("Orders length: {}", orders_len));

          if ui.button("Show All Orders").clicked() {
            self.show_all_orders = !self.show_all_orders;
          }

          let is_enabled_button_process_orders = !self.loading_process && orders_len > 0;
          ui.add_enabled_ui(is_enabled_button_process_orders, |ui| {
            if ui
                .add_sized([150.0, 30.0], Button::new("Filter & Process Orders"))
                .clicked()
            {
              self.loading_process = true;
              let tx = self.tx_process.clone();
              let orders = self.orders.clone();
              let max_price = max_price;
              let min_quantity = min_quantity;

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
          });
        });

        let processed_orders_len = self.processed_orders.as_ref().map_or(0, |orders| orders.len());
        ui.label(format!("Processed orders length: {}", processed_orders_len));

        ui.add_space(20.0);

        if self.loading_fetch {
          ui.add(Spinner::new().size(32.0));
        }

        if let Some(processed_orders) = &self.processed_orders {
          ui.label("Processed Orders:");
          ui.add_space(10.0);

          ScrollArea::new(true).show(ui, |ui| {
            for (i, order) in processed_orders.iter().enumerate() {
              let frame_stroke = if order.is_with_group.unwrap_or(false) {
                Stroke::new(2.0, ui.visuals().selection.stroke.color)
              } else {
                Stroke::new(1.0, ui.visuals().extreme_bg_color)
              };

              let message = lib::generate_message(order, offer_price);

              Frame::none()
                  .stroke(frame_stroke)
                  .rounding(Rounding::same(5))
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
      });
    });

    if self.show_settings {
      egui::Window::new("Settings")
          .open(&mut self.show_settings)
          .resizable(true)
          .show(ctx, |ui| {
            ui.horizontal(|ui| {
              if ui.button("Reset Settings to Defaults").clicked() {
                self.user_inputs = UserInputs::default();
              }

              if ui.button("Save Settings").clicked() {
                self.user_inputs.save();
              }
            });

            ui.add_space(10.0);

            // Presets section
            ui.group(|ui| {
              ui.heading("Presets");

              // Collect preset names and their current status before the UI loop
              let preset_data: Vec<(String, bool)> = self.user_inputs.presets
                  .iter()
                  .map(|preset| {
                    let is_current = self.user_inputs.current_preset_name
                        .as_ref()
                        .map_or(false, |current| current == &preset.name);
                    (preset.name.clone(), is_current)
                  })
                  .collect();

              // Show existing presets as buttons
              ui.horizontal_wrapped(|ui| {
                for (preset_name, is_current) in preset_data {
                  ui.horizontal(|ui| {
                    if ui.button(&preset_name)
                        .on_hover_text("Click to load this preset")
                        .clicked()
                    {
                      self.user_inputs.load_preset(&preset_name);
                    }

                    // Delete button for each preset
                    if ui.small_button("🗑")
                        .on_hover_text("Delete preset")
                        .clicked()
                    {
                      self.user_inputs.delete_preset(&preset_name);
                    }
                  });
                }
              });

              // Save new preset section
              ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_preset_name)
                    .on_hover_text("Enter preset name");

                if ui.button("Save as Preset")
                    .on_hover_text("Save current settings as a new preset")
                    .clicked() && !self.new_preset_name.is_empty()
                {
                  self.user_inputs.save_as_preset(self.new_preset_name.clone());
                  self.new_preset_name.clear();
                }
              });
            });

            ui.add_space(10.0);

            // Rest of the settings UI remains the same
            ui.label("Max Price:");
            if let Ok(mut value) = self.user_inputs.max_price_to_search.parse::<u32>() {
              if ui.add(DragValue::new(&mut value).clamp_range(0..=10).speed(0.02))
                  .changed()
              {
                self.user_inputs.max_price_to_search = value.to_string();
              }
            }
            ui.end_row();

            ui.label("Min Quantity:");
            if let Ok(mut value) = self.user_inputs.min_quantity_to_search.parse::<u32>() {
              if ui.add(DragValue::new(&mut value).clamp_range(0..=10).speed(0.02))
                  .changed()
              {
                self.user_inputs.min_quantity_to_search = value.to_string();
              }
            }
            ui.end_row();

            ui.label("Offer Price:");
            if let Ok(mut value) = self.user_inputs.price_to_offer.parse::<u32>() {
              if ui.add(DragValue::new(&mut value).clamp_range(0..=10).speed(0.02))
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
          });
    }

    if self.show_credits {
      egui::Window::new("Credits")
          .open(&mut self.show_credits)
          .resizable(true)
          .show(ctx, |ui| {
            ui.label("Warframe Market Ducats Buyer");
            ui.label("This app works thanks to:");
            ui.hyperlink("https://warframe.market/");
            ui.add_space(10.0);
            ui.label("Made by:");
            ui.hyperlink_to("FreePhoenix888", "https://github.com/FreePhoenix888");
            ui.hyperlink_to(
              "Source Code",
              "https://github.com/FreePhoenix888/warframe-market-ducats-buyer",
            );
            ui.add_space(10.0);
            ui.label("Special thanks to the Warframe community!");
          });
    }

    if self.show_all_orders {
      egui::Window::new("All Orders")
          .open(&mut self.show_all_orders)
          .resizable(true)
          .scroll2([true, true])
          .show(ctx, |ui| {
            if let Some(orders) = &self.orders {
              ui.label("Fetched Orders:");
              ui.add_space(10.0);

              let row_height = 50.0;
              let num_rows = orders.len();

              ScrollArea::vertical().auto_shrink([false, false]).show_rows(
                ui,
                row_height,
                num_rows,
                |ui, range| {
                  for i in range.start..range.end {
                    let order = &orders[i];

                    ui.group(|ui| {
                      ui.horizontal(|ui| {
                        ui.label("ID:");
                        ui.monospace(&order.id);
                      });

                      ui.horizontal(|ui| {
                        ui.label("Item:");
                        ui.monospace(order.item_name.as_deref().unwrap_or("Unknown"));
                        if let Some(item_url) = &order.item_url {
                          ui.hyperlink(format!(
                            "https://warframe.market/items/{}",
                            item_url
                          ));
                        }
                      });

                      ui.horizontal(|ui| {
                        ui.label("Price:");
                        ui.monospace(format!("{} platinum", order.platinum));
                      });

                      ui.horizontal(|ui| {
                        ui.label("Quantity:");
                        ui.monospace(order.quantity.to_string());
                      });

                      ui.horizontal(|ui| {
                        ui.label("User:");
                        ui.monospace(&order.user.ingame_name);
                        ui.label("(Status:");
                        ui.colored_label(
                          if order.user.status == "ingame" {
                            egui::Color32::GREEN
                          } else {
                            egui::Color32::RED
                          },
                          &order.user.status,
                        );
                        ui.label(")");
                      });

                      ui.separator();
                    });
                  }
                },
              );
            } else {
              ui.label("No orders fetched yet.");
            }
          });
    }

    self.toasts.show(ctx);
    ctx.request_repaint();
  }
}

fn main() -> eframe::Result {
  if std::env::var("RUST_LOG").is_err() {
    unsafe {
      std::env::set_var("RUST_LOG", "info");
    }
  }
  env_logger::init();
  let options = eframe::NativeOptions {
    window_builder: Some(Box::new(|builder| builder.with_maximized(true))),
    ..Default::default()
  };
  eframe::run_native(
    "Warframe Market Ducats Buyer",
    options,
    Box::new(|_cc| Ok(Box::<MyApp>::default())),
  )
}