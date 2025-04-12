#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // It's an example

mod external_lib;
mod lib;

use {
  crate::lib::{DucatsBuyer, Order},
  eframe::{
    egui,
    egui::{Align, Button, DragValue, Layout, ScrollArea, Spinner, TextEdit},
  },
  std::sync::mpsc::{self, TryRecvError},
};

#[tokio::main]
async fn main() -> eframe::Result {
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

// TODO: НЕ ДЕЛАЙ ТАК С ПУТЯМИ, НО Я ВЫНУЖДЕН ИБО ТЫ УЖЕ ЮЗАЕШЬ std::mpcs
type Mpsc<T> = (tokio::sync::mpsc::Sender<T>, tokio::sync::mpsc::Receiver<T>);

struct MyApp {
  ducats_buyer: DucatsBuyer,
  orders: Mpsc<Vec<Order>>,
  rx: mpsc::Receiver<String>,
  tx: mpsc::Sender<String>,
  loading: bool,
  user_inputs: UserInputs,
  default_inputs: UserInputs,
  error: Option<String>,
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
    // TODO: ПОТОМ НОРМАЛЬНЫЕ ШТУКИ ЗАЮЗАЙ
    let (tx1, rx1) = tokio::sync::mpsc::channel(32);
    let (tx2, rx2) = mpsc::channel();
    let default_inputs = UserInputs::default();
    Self {
      ducats_buyer: DucatsBuyer::default(),
      orders: (tx1, rx1),
      rx: rx2,
      tx: tx2,
      loading: false,
      user_inputs: default_inputs.clone(),
      default_inputs,
      error: None,
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Poll the channel for new messages without blocking the UI.
    match self.rx.try_recv() {
      Ok(error) => {
        // Successfully received a String message
        self.error = Some(error);
        self.loading = false;
      }
      Err(TryRecvError::Empty) => {
        // No new data
      }
      Err(TryRecvError::Disconnected) => {
        self.loading = false;
      }
    }
    let max_price =
      self.user_inputs.max_price_to_search.parse::<u32>().unwrap_or_default();
    let min_quantity = self
      .user_inputs
      .min_quantity_to_search
      .parse::<u32>()
      .unwrap_or_default();
    let offer_price =
      self.user_inputs.price_to_offer.parse::<u32>().unwrap_or_default();
    let item_names: Vec<String> = self
      .user_inputs
      .item_names
      .lines()
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();

    // А зачем каждый фрейм?
    // self.ducats_buyer = self
    //   .ducats_buyer
    //   .with_item_names(item_names)
    //   .with_filter(move |order| {
    //     // Add 'move' keyword here
    //     return order.user.status == "ingame"
    //       && order.order_type == "sell"
    //       && order.visible
    //       && order.quantity >= min_quantity
    //       && order.platinum <= max_price;
    //   })
    //   .with_desired_price(offer_price);

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
        ui.heading("My egui Application with Async Orders");
        ui.add_space(20.0);

        // Input fields section
        egui::Grid::new("input_grid").num_columns(3).show(ui, |ui| {
          ui.label("Max Price:");
          if let Ok(mut value) =
            self.user_inputs.max_price_to_search.parse::<i32>()
          {
            if ui
              .add(
                DragValue::new(&mut value).clamp_range(0..=i32::MAX).speed(1),
              )
              .changed()
            {
              self.user_inputs.max_price_to_search = value.to_string();
            }
          }
          ui.end_row();

          ui.label("Min Quantity:");
          if let Ok(mut value) =
            self.user_inputs.min_quantity_to_search.parse::<i32>()
          {
            if ui
              .add(
                DragValue::new(&mut value).clamp_range(0..=i32::MAX).speed(1),
              )
              .changed()
            {
              self.user_inputs.min_quantity_to_search = value.to_string();
            }
          }
          ui.end_row();

          ui.label("Offer Price:");
          if let Ok(mut value) = self.user_inputs.price_to_offer.parse::<i32>()
          {
            if ui
              .add(
                DragValue::new(&mut value).clamp_range(0..=i32::MAX).speed(1),
              )
              .changed()
            {
              self.user_inputs.price_to_offer = value.to_string();
            }
          }
          ui.end_row();
        });

        ui.add_space(10.0);

        ui.label("Item Names (one per line):");
        ui.add(
          TextEdit::multiline(&mut self.user_inputs.item_names)
            .hint_text("Enter item names (one per line)")
            .desired_width(f32::INFINITY)
            .min_size([ui.available_width(), 100.0].into()),
        );

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
          if ui.button("Reset to Defaults").clicked() {
            self.user_inputs = self.default_inputs.clone();
          }

          if ui.add_sized([150.0, 30.0], Button::new("Fetch Orders")).clicked()
            && !self.loading
          {
            self.loading = true;
            let err = self.tx.clone();
            let ord = self.orders.0.clone(); // TODO: не важно, просто юзай Promise или что там рекомендуют парни

            let items = self.ducats_buyer.items.clone();
            // TODO: Я БЫ ЭТО ОСТАВИЛ НА СТОРОНЕ `App` ИБО ЭТО ТО, ЧТО МЕНЕДЖИТ ЮЗЕР
            //  ДА И ФИЛЬТРОВАТЬСЯ ОНО БУДЕТ ВСЁ РАВНО НА СТОРОНЕ КЛИЕНТА
            let filter = self.ducats_buyer.filter().clone();
            tokio::spawn(async move {
              match DucatsBuyer::fetch_orders(&items, &*filter).await {
                Ok(orders) => {
                  let _ = ord.send(orders).await;
                }
                Err(e) => {
                  // NOTE: НУ ТЫ ЭТО, ОПРЕДЕЛИСЬ ТЫ ЛИБО СТОПАЕШЬ, ЛИБО В КАНАЛ ПОСТИШЬ
                  let _ = err.send(format!("{e:?}"));
                }
              };
            });
          }
        });

        ui.add_space(20.0);

        if self.loading {
          ui.add(Spinner::new().size(32.0));
        }

        // Обработчки тасковых штук можно вниз перенести - ну короче в одно место
        if let Ok(orders) = self.orders.1.try_recv() {
          self.loading = false;
          self.ducats_buyer.orders = orders; // мэйби сделать так, чтобы адейт ордеров сразу вызывал процесс
          let _ = self.ducats_buyer.process_orders(); // мне лень обрабатывать, хз что может сломаться
        }

        let processed_orders = self.ducats_buyer.get_processed_orders();
        if processed_orders.len() > 0 {
          ui.label("Fetched Order Messages:");
          ui.add_space(10.0);

          ScrollArea::new(true).show(ui, |ui| {
            for (i, order) in processed_orders.iter().enumerate() {
              let bg_color = if order.is_with_group.unwrap_or(false) {
                ui.visuals().selection.bg_fill
              } else {
                ui.visuals().extreme_bg_color
              };

              let message = DucatsBuyer::generate_message(order);

              let button =
                ui.add_sized([100.0, 100.0], Button::new(message.clone()));
              if button.clicked() {
                ui.ctx().copy_text(message.clone());
              }

              ui.add_space(8.0);
            }
          });
        }

        if let Some(error_message) = &self.error {
          ui.colored_label(ui.visuals().warn_fg_color, error_message);
        }
      });
    });

    ctx.request_repaint();
  }
}
