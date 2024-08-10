pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::OrderbookAdapterInterface;
pub use error::OrderbookAdapterError;
pub use msg::{OrderbookAdapterExecuteMsg, OrderbookAdapterInstantiateMsg};

/// The version of your Adapter
pub const ADAPTER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ORDERBOOK_NAMESPACE: &str = "thxsh";
pub const ORDERBOOK_ADAPTER_NAME: &str = "orderbook-adapter";
pub const ORDERBOOK_ADAPTER_ID: &str =
    const_format::concatcp!(ORDERBOOK_NAMESPACE, ":", ORDERBOOK_ADAPTER_NAME);
