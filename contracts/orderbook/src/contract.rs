use crate::{
    error::OrderbookError,
    handlers,
    msg::{OrderbookExecuteMsg, OrderbookInstantiateMsg, OrderbookMigrateMsg, OrderbookQueryMsg},
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, ORDERBOOK_ID,
};

use abstract_app::AppContract;
use cosmwasm_std::Response;

/// The type of the result returned by your app's entry points.
pub type OrderbookResult<T = Response> = Result<T, OrderbookError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type Orderbook = AppContract<
    OrderbookError,
    OrderbookInstantiateMsg,
    OrderbookExecuteMsg,
    OrderbookQueryMsg,
    OrderbookMigrateMsg,
>;

const APP: Orderbook = Orderbook::new(ORDERBOOK_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_migrate(handlers::migrate_handler)
    .with_module_ibc(handlers::ibc_handler)
    .with_dependencies(&[])
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, Orderbook);

abstract_app::cw_orch_interface!(APP, Orderbook, OrderbookInterface);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::OrderbookInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
