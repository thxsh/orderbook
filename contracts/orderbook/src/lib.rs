pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;

#[cfg(test)]
mod tests;

pub use error::OrderbookError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::OrderbookInterface;

pub const ORDERBOOK_NAMESPACE: &str = "thxsh";
pub const ORDERBOOK_NAME: &str = "orderbook";
pub const ORDERBOOK_ID: &str = const_format::concatcp!(ORDERBOOK_NAMESPACE, ":", ORDERBOOK_NAME);
