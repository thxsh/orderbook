use crate::{
    msg::{OrderbookAdapterExecuteMsg, OrderbookAdapterQueryMsg},
    ORDERBOOK_ADAPTER_ID,
};

use abstract_adapter::sdk::{
    features::{AccountIdentification, Dependencies, ModuleIdentification},
    AbstractSdkResult, AdapterInterface,
};
use abstract_adapter::std::objects::module::ModuleId;
use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{CosmosMsg, Deps, Uint128};

// API for Abstract SDK users
/// Interact with your adapter in other modules.
pub trait OrderbookAdapterApi: AccountIdentification + Dependencies + ModuleIdentification {
    /// Construct a new adapter interface.
    fn orderbook_adapter<'a>(&'a self, deps: Deps<'a>) -> OrderbookAdapter<Self> {
        OrderbookAdapter {
            base: self,
            deps,
            module_id: ORDERBOOK_ADAPTER_ID,
        }
    }
}

impl<T: AccountIdentification + Dependencies + ModuleIdentification> OrderbookAdapterApi for T {}

#[derive(Clone)]
pub struct OrderbookAdapter<'a, T: OrderbookAdapterApi> {
    pub base: &'a T,
    pub module_id: ModuleId<'a>,
    pub deps: Deps<'a>,
}

impl<'a, T: OrderbookAdapterApi> OrderbookAdapter<'a, T> {
    /// Set the module id
    pub fn with_module_id(self, module_id: ModuleId<'a>) -> Self {
        Self { module_id, ..self }
    }

    /// returns the HUB module id
    fn module_id(&self) -> ModuleId {
        self.module_id
    }

    /// Executes a [OrderbookAdapterExecuteMsg] in the adapter
    fn request(&self, msg: OrderbookAdapterExecuteMsg) -> AbstractSdkResult<CosmosMsg> {
        let adapters = self.base.adapters(self.deps);

        adapters.execute(self.module_id(), msg)
    }

    /// Route message
    pub fn update_config(&self) -> AbstractSdkResult<CosmosMsg> {
        self.request(OrderbookAdapterExecuteMsg::UpdateConfig {})
    }
}

/// Queries
impl<'a, T: OrderbookAdapterApi> OrderbookAdapter<'a, T> {
    /// Query your adapter via message type
    pub fn query<R: DeserializeOwned>(
        &self,
        query_msg: OrderbookAdapterQueryMsg,
    ) -> AbstractSdkResult<R> {
        let adapters = self.base.adapters(self.deps);
        adapters.query(self.module_id(), query_msg)
    }

    /// Query config
    pub fn config(&self) -> AbstractSdkResult<Uint128> {
        self.query(OrderbookAdapterQueryMsg::Config {})
    }
}
