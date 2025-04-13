use convert_case::{Case, Casing};
use fake::{Fake, Faker};
use serde::Deserialize;
use serde::Serialize;
use std::cmp;
use std::collections::HashMap;

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
    pub platinum: i32,
    pub quantity: i32,
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
    pub price_to_offer: Option<i32>,
    #[serde(skip_deserializing)]
    pub sum_to_offer: Option<i32>,
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

pub const MIN_QUANTITY_TO_SEARCH: i32 = 2;
pub const MAX_PRICE_TO_SEARCH: i32 = 4;

pub const PRICE_TO_OFFER: i32 = 3;

pub fn default_order_filter(order: &Order) -> bool {
    let is_order_profitable = order.user.status == "ingame"
        && order.order_type == "sell"
        && order.visible
        && order.platinum <= MAX_PRICE_TO_SEARCH
        && order.quantity >= MIN_QUANTITY_TO_SEARCH;

    is_order_profitable
}

pub const PROFITABLE_ITEM_NAMES: [&str; 34] = [
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
    "Stradavar Prime Barrel",
    "Ninkondi Prime Chain",
    "Zakti Prime Barrel",
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

const DESIRED_PRICE: i32 = 3;

/// Fetches all orders for the given item names.
pub async fn fetch_all_orders(
    item_names: &[String],
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let mut orders: Vec<Order> = Vec::new();

    for item_name in item_names {
        let item_url = item_name.to_case(Case::Snake);
        let get_orders_response =
            reqwest::get(BASE_URL.to_owned() + "/items/" + &item_url + "/orders")
                .await?
                .json::<GetOrdersResponse>()
                .await?;

        let enriched_orders: Vec<Order> = get_orders_response
            .payload
            .orders
            .into_iter()
            .map(|mut order| {
                order.item_name = Some(item_name.to_string());
                order.item_url = Some(item_url.to_string());
                order
            })
            .collect();

        for order in enriched_orders {
            orders.push(order);
        }
    }

    Ok(orders)
}

/// Filters orders based on the provided filter function.
pub fn filter_orders(orders: Vec<Order>, filter: impl Fn(&Order) -> bool) -> Vec<Order> {
    let mut filtered_orders = vec![];

    for order in orders {
        if filter(&order) {
            filtered_orders.push(order.clone());
        }
    }

    filtered_orders
}

/// Processes the filtered orders by enriching fields and sorting them.
pub fn process_orders(orders: Vec<Order>) -> Vec<Order> {
    let mut grouped_orders: HashMap<String, Vec<Order>> = HashMap::new();

    for mut order in orders {
        order.is_with_group = Some(false);
        order.price_to_offer = Some(cmp::min(PRICE_TO_OFFER, order.platinum));
        order.sum_to_offer = Some(order.price_to_offer.unwrap() * order.quantity);
        grouped_orders
            .entry(order.user.ingame_name.clone())
            .or_default()
            .push(order);
    }

    // Mark orders with group
    grouped_orders
        .values_mut()
        .filter(|orders| orders.len() > 1)
        .for_each(|orders| {
            for order in orders {
                order.is_with_group = Some(true);
            }
        });

    // Group by profit and sort
    let mut processed_orders: Vec<_> = grouped_orders
        .into_iter()
        .map(|(user_name, mut orders)| {
            orders.sort_by_key(|o| std::cmp::Reverse(o.quantity));
            (user_name, orders)
        })
        .collect();

    processed_orders.sort_by_key(|(_, orders)| {
        std::cmp::Reverse(orders.iter().map(|o| o.quantity).sum::<i32>())
    });

    // Flatten to final list
    processed_orders
        .into_iter()
        .flat_map(|(_, orders)| orders)
        .collect()
}

/// Generates a message for a single order.
pub fn generate_message(order: &Order, desired_price: i32) -> String {
    let user = &order.user.ingame_name;
    let platinum = order.platinum;
    let quantity = order.quantity;
    let price_to_offer = cmp::min(desired_price, platinum);
    let total_price = price_to_offer * quantity;
    let item_name = order.item_name.as_ref().unwrap();

    if quantity == 1 {
        format!(
            "/w {user} Hi, {user}! \
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy! :)",
            user = user,
            item_name = item_name,
            platinum = platinum,
        )
    } else if price_to_offer == platinum {
        format!(
            "/w {user} Hi, {user}! \
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy all {quantity} pieces",
            user = user,
            item_name = item_name,
            platinum = platinum,
            quantity = quantity,
        )
    } else {
        format!(
            "/w {user} Hi, {user}! \
                You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
                I want to buy all with discount {price_to_offer}:platinum:*{quantity}={total_price}:platinum: if you are interested :)",
            user = user,
            item_name = item_name,
            platinum = platinum,
            price_to_offer = price_to_offer,
            quantity = quantity,
            total_price = total_price
        )
    }
}

/// Generates messages for all processed orders.
pub fn generate_messages(orders: &[Order], desired_price: i32) -> Vec<String> {
    orders
        .iter()
        .map(|order| generate_message(order, desired_price))
        .collect()
}
