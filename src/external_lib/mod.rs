pub mod external;
pub mod mock;

#[cfg(not(feature = "mock"))]
pub use external::fetch_all_profitable_orders;
#[cfg(feature = "mock")]
pub use mock::fetch_all_profitable_orders;

#[cfg(not(feature = "mock"))]
pub use external::fetch_all_profitable_orders_with_filter;
#[cfg(feature = "mock")]
pub use mock::fetch_all_profitable_orders_with_filter;

#[cfg(not(feature = "mock"))]
pub use external::fetch_all_profitable_orders_with_filter_with_item_names;
#[cfg(feature = "mock")]
pub use mock::fetch_all_profitable_orders_with_filter_with_item_names;




pub use external::generate_message;
pub use external::Order;
pub use external::User;
pub use external::default_order_filter;
pub use external::PROFITABLE_ITEM_NAMES;