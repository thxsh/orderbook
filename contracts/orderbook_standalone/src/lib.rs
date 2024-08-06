pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

use abstract_standalone::StandaloneContract;
use cosmwasm_std::Response;
pub use error::OrderbookStandaloneError;

/// The version of your standalone
pub const STANDALONE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const ORDERBOOK_NAMESPACE: &str = "orderbook";
pub const ORDERBOOK_STANDALONE_NAME: &str = "orderbook-standalone";
pub const ORDERBOOK_STANDALONE_ID: &str = const_format::concatcp!(ORDERBOOK_NAMESPACE, ":", ORDERBOOK_STANDALONE_NAME);

/// The type of the result returned by your standalone's entry points.
pub type OrderbookStandaloneResult<T = Response> = Result<T, OrderbookStandaloneError>;

/// The type of the standalone that is used to build your contract object and access the Abstract SDK features.
pub type OrderbookStandalone = StandaloneContract;

pub const ORDERBOOK_STANDALONE: OrderbookStandalone =
    OrderbookStandalone::new(ORDERBOOK_STANDALONE_ID, STANDALONE_VERSION, None);

// cw-orch related interface
#[cfg(not(target_arch = "wasm32"))]
mod interface;

#[cfg(not(target_arch = "wasm32"))]
pub use interface::OrderbookStandaloneInterface;
