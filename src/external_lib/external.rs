use std::sync::Arc;
use {
  convert_case::{Case, Casing},
  fake::{Fake, Faker},
  serde::{Deserialize, Serialize},
  std::{cmp, collections::HashMap, error::Error}, // anyhow::Error неплох
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrdersResponse {
  pub payload: Payload,
  // pub include: Include,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
  pub orders: Vec<Order>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
  pub id: String,
  pub platinum: u32,
  pub quantity: u32,
  pub order_type: String,
  // pub platform: String,
  // pub region: String,
  // pub creation_date: String,
  // pub last_update: String,
  // pub subtype: String,
  pub visible: bool,
  pub user: User,

  #[serde(skip_deserializing)]
  pub item_url: Option<String>,
  #[serde(skip_deserializing)]
  pub item_name: Option<String>,
  #[serde(skip_deserializing)]
  pub price_to_offer: Option<u32>,
  #[serde(skip_deserializing)]
  pub sum_to_offer: Option<u32>,
  #[serde(skip_deserializing)]
  pub is_with_group: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
  // pub id: String,
  pub ingame_name: String,
  pub status: String,
  // pub region: String,
  // pub reputation: i64,
  // pub avatar: String,
  // pub last_seen: String,
  #[serde(skip_deserializing)]
  pub profitable_orders_count: Option<i32>,
}

impl Default for User {
  fn default() -> Self {
    User {
      ingame_name: Faker.fake(),
      status: Faker.fake(),
      profitable_orders_count: Faker.fake(),
    }
  }
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Include {
//     pub item: Item,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Item {
//     pub id: String,
//     pub items_in_set: Vec<ItemsInSet>,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct ItemsInSet {
//     pub id: String,
//     pub url_name: String,
//     pub icon: String,
//     pub icon_format: String,
//     pub thumb: String,
//     pub sub_icon: String,
//     pub mod_max_rank: i64,
//     pub subtypes: Vec<String>,
//     pub tags: Vec<String>,
//     pub ducats: i64,
//     pub quantity_for_set: i64,
//     pub set_root: bool,
//     pub mastery_level: i64,
//     pub rarity: String,
//     pub trading_tax: i64,
//     pub en: En,
//     pub ru: Ru,
//     pub ko: Ko,
//     pub fr: Fr,
//     pub de: De,
//     pub sv: Sv,
//     pub zh_hant: ZhHant,
//     pub zh_hans: ZhHans,
//     pub pt: Pt,
//     pub es: Es,
//     pub pl: Pl,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct En {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ru {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop2 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ko {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop3 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fr {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop4 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct De {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop5 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sv {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop6 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZhHant {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop7 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZhHans {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop8 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pt {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop9>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop9 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Es {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop10>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop10 {
  pub name: String,
  pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pl {
  pub item_name: String,
  pub description: String,
  pub wiki_link: String,
  pub drop: Vec<Drop11>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Drop11 {
  pub name: String,
  pub link: String,
}

const BASE_URL: &str = "https://api.warframe.market/v1";

pub const MIN_QUANTITY_TO_SEARCH: u32 = 2;
pub const MAX_PRICE_TO_SEARCH: u32 = 4;

pub const PRICE_TO_OFFER: u32 = 3;

pub fn default_order_filter(order: &Order) -> bool {
  let is_order_profitable = order.user.status == "ingame"
    && order.order_type == "sell"
    && order.visible
    && order.platinum <= MAX_PRICE_TO_SEARCH
    && order.quantity >= MIN_QUANTITY_TO_SEARCH;

  is_order_profitable
}

pub const PROFITABLE_ITEM_NAMES: [&str; 36] = [
  "Harrow Prime Blueprint",
  "Astilla Prime Stock",
  "Braton Prime Receiver",
  "Knell Prime Receiver",
  "Corvas Prime Blueprint",
  "Magnus Prime Receiver",
  "Burston Prime Barrel",
  "Akbronco Prime Link",
  "Pandero Prime Barrel",
  "Nagantaka Prime Stock",
  "Scourge Prime Handle",
  "Tekko Prime Blueprint",
  "Orthos Prime Blueprint",
  "Zakti Prime Barrel",
  "Stradavar Prime Barrel",
  "Ninkondi Prime Chain",
  "Zakti Prime Barrel",
  "Ninkondi Prime Chain",
  "Afuris Prime Link",
  "Nidus Prime Blueprint",
  "Baza Prime Barrel",
  "Harrow Prime Neuroptics Blueprint",
  "Inaros Prime Chassis Blueprint",
  "Gara Prime Neuroptics Blueprint",
  "Karyst Prime Handle",
  "Tatsu Prime Blade",
  "Volnus Prime Head",
  "Redeemer Prime Blueprint",
  "Dethcube Prime Carapace",
  "Titania Prime Neuroptics Blueprint",
  "Guandao Prime Blueprint",
  "Garuda Prime Chassis Blueprint",
  "Panthera Prime Stock",
  "Khora Prime Chassis Blueprint",
  "Atlas Prime Chassis Blueprint",
  "Dual Keres Prime Blueprint",
];

const DESIRED_PRICE: u32 = 3;

pub struct DucatsBuyer {
  pub items: Vec<String>,
  /// Desired price of items
  pub price: u32,
  pub orders: Vec<Order>,
  processed_orders: Vec<Order>,
  filter: Arc<dyn Fn(&Order) -> bool + Sync + Send>,
}

impl Default for DucatsBuyer {
  fn default() -> Self {
    Self {
      items: PROFITABLE_ITEM_NAMES.iter().map(<_>::to_string).collect(),
      price: DESIRED_PRICE,
      filter: Arc::new(default_order_filter),
      orders: vec![],
      processed_orders: vec![],
    }
  }
}

impl DucatsBuyer {
  // не ну ряльно долго писать этот тип
  pub fn filter(&self) -> Arc<dyn Fn(&Order) -> bool + Sync + Send> {
    self.filter.clone()
  }

  pub fn with_item_names(mut self, item_names: Vec<String>) -> Self {
    self.items = item_names;
    self
  }

  pub fn with_desired_price(mut self, price: u32) -> Self {
    self.price = price;
    self
  }

  pub fn with_filter(
    mut self,
    filter: impl Fn(&Order) -> bool + Sync + Send + 'static,
  ) -> Self {
    self.filter = Arc::new(filter);
    self
  }

  pub async fn fetch_order(
    item: &str,
    filter: impl Fn(&Order) -> bool,
  ) -> Result<Vec<Order>, Box<dyn Error + Send + Sync>> {
    let url = item.to_case(Case::Snake);
    let GetOrdersResponse { payload: Payload { orders } } =
      reqwest::get(format!("{BASE_URL}/items/{url}/orders"))
        .await?
        .json()
        .await?;

    let profitable_orders: Vec<_> = orders
      .into_iter()
      // TODO: НО! Я БЫ ВЫНЕС ЭТО ОТДЕЛЬНО, ТО ЕСТЬ ФИЛЬТРОВАТЬ В UI ПОТОКЕ
      .filter(|order| (filter)(order))
      .map(|mut order| {
        order.item_name = Some(item.to_string());
        order.item_url = Some(url.to_string());
        order
      })
      .collect();

    Ok(profitable_orders)
  }

  pub async fn fetch_orders(
    items: &[String],
    filter: impl Fn(&Order) -> bool,
  ) -> Result<Vec<Order>, Box<dyn Error + Send + Sync>> {
    let mut orders = Vec::new();
    for item in items {
      orders.extend(Self::fetch_order(item, &filter).await?);
    }
    Ok(orders)
  }

  pub fn process_orders(&mut self) -> Result<(), Box<dyn Error>> {
    let mut user_orders: HashMap<String, Vec<Order>> = HashMap::new();

    // Group orders by user
    for order in &self.orders {
      user_orders
        .entry(order.user.ingame_name.clone())
        .or_default()
        .push(order.clone());
    }

    // Enrich custom fields
    user_orders.values_mut().for_each(|orders| {
      for order in orders.iter_mut() {
        order.is_with_group = Some(false);
        order.price_to_offer = Some(cmp::min(self.price, order.platinum));
        order.sum_to_offer =
          Some(order.price_to_offer.unwrap() * order.quantity);
      }
    });

    // Mark orders with group
    user_orders.values_mut().filter(|orders| orders.len() > 1).for_each(
      |orders| {
        for order in orders.iter_mut() {
          order.is_with_group = Some(true);
        }
      },
    );

    // Grouped and sorted by user profit
    let mut grouped: Vec<_> = user_orders
      .into_iter()
      .map(|(user_name, mut orders)| {
        orders.sort_by_key(|o| std::cmp::Reverse(o.sum_to_offer.unwrap()));
        (user_name, orders)
      })
      .collect();

    grouped.sort_by_key(|(_, orders)| {
      std::cmp::Reverse(
        orders.iter().map(|o| o.sum_to_offer.unwrap()).sum::<u32>(),
      )
    });

    // Flatten to final list
    let processed_orders =
      grouped.into_iter().flat_map(|(_, orders)| orders).collect();

    self.processed_orders = processed_orders;

    Ok(())
  }

  pub fn get_orders(&self) -> &[Order] {
    &self.orders
  }

  pub fn get_processed_orders(&self) -> &[Order] {
    &self.processed_orders
  }

  /// Generates a message for a single order
  pub fn generate_message(order: &Order) -> String {
    let user = &order.user.ingame_name;
    let item_url = order.item_url.as_ref().unwrap();
    let platinum = order.platinum;
    let quantity = order.quantity;
    let desired_price = order.price_to_offer.unwrap();
    let desired_sum = order.sum_to_offer.unwrap();
    let item_name = item_url.to_case(Case::Pascal);

    if quantity == 1 {
      format!(
        "/w {user} Hi, {user}!\
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy! :)",
        user = user,
        item_name = item_name,
        platinum = platinum,
      )
    } else if desired_price == platinum {
      format!(
        "/w {user} Hi, {user}!\
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy all {quantity} pieces",
        user = user,
        item_name = item_name,
        platinum = platinum,
        quantity = quantity,
      )
    } else {
      format!(
        "/w {user} Hi, {user}!\
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy all with discount {desired_price}:platinum:*{quantity}={desired_sum}:platinum: if you are interested :)",
        user = user,
        item_name = item_name,
        platinum = platinum,
        desired_price = desired_price,
        quantity = quantity,
        desired_sum = desired_sum
      )
    }
  }

  /// Generates messages for all stored orders
  pub fn generate_messages(&self) -> Vec<String> {
    self.orders.iter().map(Self::generate_message).collect()
  }
}
