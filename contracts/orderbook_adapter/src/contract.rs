use crate::{
    error::OrderbookAdapterError,
    handlers,
    msg::{OrderbookAdapterExecuteMsg, OrderbookAdapterInstantiateMsg, OrderbookAdapterQueryMsg},
    ADAPTER_VERSION, ORDERBOOK_ADAPTER_ID,
};

use abstract_adapter::AdapterContract;
use cosmwasm_std::Response;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type OrderbookAdapter = AdapterContract<
    OrderbookAdapterError,
    OrderbookAdapterInstantiateMsg,
    OrderbookAdapterExecuteMsg,
    OrderbookAdapterQueryMsg,
>;
/// The type of the result returned by your Adapter's entry points.
pub type AdapterResult<T = Response> = Result<T, OrderbookAdapterError>;

const ORDERBOOK_ADAPTER: OrderbookAdapter =
    OrderbookAdapter::new(ORDERBOOK_ADAPTER_ID, ADAPTER_VERSION, None)
        .with_instantiate(handlers::instantiate_handler)
        .with_execute(handlers::execute_handler)
        .with_query(handlers::query_handler);

// Export handlers
#[cfg(feature = "export")]
abstract_adapter::export_endpoints!(ORDERBOOK_ADAPTER, OrderbookAdapter);

abstract_adapter::cw_orch_interface!(
    ORDERBOOK_ADAPTER,
    OrderbookAdapter,
    OrderbookAdapterInstantiateMsg,
    OrderbookAdapterInterface
);
