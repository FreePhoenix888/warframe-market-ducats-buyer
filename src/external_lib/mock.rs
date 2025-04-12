#[cfg(feature = "mock")]
use rand::prelude::*;
use rand::Rng;
use tokio::sync::mpsc;
use crate::external_lib::Order;
use crate::lib;

pub async fn fetch_all_profitable_orders() -> Result<Vec<lib::Order>, Box<dyn std::error::Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let mut rng = rand::rng();
    let shared_user: lib::User = Default::default();


    let mut make_order = |use_shared_user: bool, rng: &mut rand::rngs::ThreadRng| -> lib::Order {
        let user = if use_shared_user { shared_user.clone() } else { Default::default() };
        lib::Order {
            id: rng.random_range(100..=1000).to_string(),
            platinum: rng.random_range(0..=1000),
            quantity: rng.random_range(1..=10),
            order_type: if rng.random_bool(0.5) { "buy".into() } else { "sell".into() },
            visible: rng.random_bool(0.8),
            user,
            item_name: Some(rng.random_range(1..=1000).to_string()),
            item_url: Some(rng.random_range(1..=1000).to_string()),
            desired_price: Some(rng.random_range(0..=1000)),
            desired_sum: Some(rng.random_range(0..=1000)),
            is_with_group: Some(use_shared_user),
        }
    };


    let mut orders = Vec::with_capacity(5);
    orders.push(make_order(false, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(false, &mut rng));

    while orders.len() < 20 {
        orders.push(make_order(false, &mut rng));
    }


    Ok(orders)
}

pub async fn fetch_all_profitable_orders_with_filter(
    filter_fn: impl Fn(&Order) -> bool
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let item_names: Vec<String> = crate::external_lib::external::PROFITABLE_ITEM_NAMES.iter().map(|&s| {s.to_string()}).collect();
    crate::external_lib::external::fetch_all_profitable_orders_with_filter_with_item_names(
        item_names,
        filter_fn,
    ).await
}

pub async fn fetch_all_profitable_orders_with_filter_with_item_names(
    item_names: Vec<String>,
    filter_fn: impl Fn(&Order) -> bool
) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let mut rng = rand::rng();
    let shared_user: lib::User = Default::default();


    let mut make_order = |use_shared_user: bool, rng: &mut rand::rngs::ThreadRng| -> lib::Order {
        let user = if use_shared_user { shared_user.clone() } else { Default::default() };
        lib::Order {
            id: rng.random_range(100..=1000).to_string(),
            platinum: rng.random_range(0..=1000),
            quantity: rng.random_range(1..=10),
            order_type: if rng.random_bool(0.5) { "buy".into() } else { "sell".into() },
            visible: rng.random_bool(0.8),
            user,
            item_name: Some(rng.random_range(1..=1000).to_string()),
            item_url: Some(rng.random_range(1..=1000).to_string()),
            desired_price: Some(rng.random_range(0..=1000)),
            desired_sum: Some(rng.random_range(0..=1000)),
            is_with_group: Some(use_shared_user),
        }
    };


    let mut orders = Vec::with_capacity(5);
    orders.push(make_order(false, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(true, &mut rng));
    orders.push(make_order(false, &mut rng));

    while orders.len() < 20 {
        orders.push(make_order(false, &mut rng));
    }


    Ok(orders)
}


