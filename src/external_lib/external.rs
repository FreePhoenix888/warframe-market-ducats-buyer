use std::cmp;
use std::collections::HashMap;
use convert_case::{Case, Casing};
use fake::{Fake, Faker};
use serde::Serialize;
use serde::Deserialize;
use tokio::sync::mpsc;
use crate::lib;

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
    pub platinum: i64,
    pub quantity: i64,
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
    pub desired_price: Option<i64>,
    #[serde(skip_deserializing)]
    pub desired_sum: Option<i64>,
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

pub fn default_order_filter(order: &Order) -> bool {
    let is_order_profitable = order.user.status == "ingame"
    && order.order_type == "sell"
    && order.visible
    && order.platinum <= 4
    && order.quantity >= 2;

    is_order_profitable
 }

async fn fetch_profitable_orders_with_filter(
    item_name: &str,
    filter_fn: impl Fn(&Order) -> bool
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {

    let get_orders_response = reqwest::get(BASE_URL.to_owned() + "/items/" + item_name + "/orders")
        .await?
        .json::<GetOrdersResponse>()
        .await?;

    let profitable_orders: Vec<Order> = get_orders_response.payload.orders
        .iter()
        .filter(|&order| filter_fn(order))
        .map(|order| {
            let mut enriched_order = order.clone();
            enriched_order.item_name = Some(item_name.to_string());
            let item_url = item_name.to_case(Case::Snake);
            enriched_order.item_url = Some(item_url.to_string());
            enriched_order
        })
        .collect();

    Ok(profitable_orders)
}

pub fn generate_message(order: &Order) -> String {
    let user = &order.user.ingame_name;
    let item_url = order.item_url.as_ref().unwrap();
    let platinum = order.platinum;
    let quantity = order.quantity;
    let desired_price = order.desired_price.unwrap();
    let desired_sum = order.desired_sum.unwrap();
    let item_name = item_url.to_case(Case::Pascal);

    // TODO: use item_name
    if quantity == 1 {
        format!(
            "/w {user} Hi, {user}!\
        You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
        I want to buy! :)",
            user = user,
            item_name = item_name, platinum= platinum,
        )
    } else {
        if desired_price == platinum {
            format!(
                "/w {user} Hi, {user}!\
        You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
        I want to buy all {quantity} pieces",
                user = user,
                item_name = item_name, platinum= platinum,
                quantity=quantity,
            )
        } else {
            format!(
                "/w {user} Hi, {user}!\
        You have WTS order: {item_name} for {platinum} :platinum: for each on warframe.market. \
        I want to buy all with discount {desired_price}:platinum:*{quantity}={desired_sum}:platinum: if you are interested :)",
                user = user,
                item_name = item_name, platinum= platinum,
                desired_price=desired_price,
                quantity=quantity,
                desired_sum=desired_sum
            )
        }
    }


}

pub async fn fetch_all_profitable_orders(
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let filter_fn = default_order_filter;
    fetch_all_profitable_orders_with_filter(
        filter_fn,
    ).await
}

pub async fn fetch_all_profitable_orders_with_item_names(
    item_names: Vec<String>,
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let filter = default_order_filter;
    fetch_all_profitable_orders_with_filter_with_item_names(
        item_names,
        filter,
    ).await
}

pub async fn fetch_all_profitable_orders_with_filter(
    filter_fn: impl Fn(&Order) -> bool
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let item_names: Vec<String> = PROFITABLE_ITEM_NAMES.iter().map(|&s| {s.to_string()}).collect();
    fetch_all_profitable_orders_with_filter_with_item_names(
        item_names,
        filter_fn,
    ).await
}

pub async fn fetch_all_profitable_orders_with_filter_with_item_names(
    item_names: Vec<String>,
    filter_fn: impl Fn(&Order) -> bool
) -> Result<Vec<Order>, Box<dyn std::error::Error>>
{
    let mut user_orders: HashMap<String, Vec<Order>> = HashMap::new();

    for item_name in item_names {
        let profitable_orders = fetch_profitable_orders_with_filter(&item_name, |order| {filter_fn(order)}).await?;
        for order in profitable_orders {
            user_orders
                .entry(order.user.ingame_name.clone())
                .or_default()
                .push(order);
        }
    }

    // Enrich custom fields
    user_orders.values_mut().for_each(|orders| {
        for order in orders.iter_mut() {
            order.is_with_group = Some(false);
            order.desired_price = Some(cmp::min(3, order.platinum));
            order.desired_sum = Some(order.desired_price.unwrap() * order.quantity);
        }
    });

    // Mark orders with group
    user_orders
        .values_mut()
        .filter(|orders| orders.len() > 1)
        .for_each(|orders| {
            for order in orders.iter_mut() {
                order.is_with_group = Some(true);
            }
        });

    // Grouped and sorted by user profit
    let mut grouped: Vec<_> = user_orders
        .into_iter()
        .map(|(user_name, mut orders)| {
            orders.sort_by_key(|o| std::cmp::Reverse(o.desired_sum.unwrap()));
            (user_name, orders)
        })
        .collect();

    grouped.sort_by_key(|(_, orders)| {
        std::cmp::Reverse(orders.iter().map(|o| o.desired_sum.unwrap()).sum::<i64>())
    });

    // Flatten to final list
    let final_orders: Vec<Order> = grouped
        .into_iter()
        .flat_map(|(_, orders)| orders)
        .collect();

    Ok(final_orders)
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