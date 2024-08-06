use cw_orch::contract::{interface_traits::InstantiableContract, Contract};
use cw_orch::prelude::*;

use crate::{
    msg::*, ORDERBOOK_STANDALONE
};

#[cw_orch::interface(
    OrderbookStandaloneInstantiateMsg,
    OrderbookStandaloneExecuteMsg,
    OrderbookStandaloneQueryMsg,
    OrderbookStandaloneMigrateMsg
)]
pub struct OrderbookStandaloneInterface;

impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for OrderbookStandaloneInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}

impl<Chain: cw_orch::environment::CwEnv> abstract_interface::RegisteredModule
    for OrderbookStandaloneInterface<Chain>
{
    type InitMsg = <OrderbookStandaloneInterface<Chain> as InstantiableContract>::InstantiateMsg;

    fn module_id<'a>() -> &'a str {
        ORDERBOOK_STANDALONE.module_id()
    }

    fn module_version<'a>() -> &'a str {
        ORDERBOOK_STANDALONE.version()
    }
}

impl<Chain: cw_orch::environment::CwEnv> From<Contract<Chain>> for OrderbookStandaloneInterface<Chain> {
    fn from(value: Contract<Chain>) -> Self {
        OrderbookStandaloneInterface(value)
    }
}

impl<Chain: cw_orch::environment::CwEnv> Uploadable for OrderbookStandaloneInterface<Chain> {
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        let wasm_name = env!("CARGO_CRATE_NAME").replace('-', "_");
        cw_orch::prelude::ArtifactsDir::auto(Some(env!("CARGO_MANIFEST_DIR").to_string()))
            .find_wasm_path(&wasm_name)
            .unwrap()
    }

    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
            .with_migrate(crate::contract::migrate),
        )
    }
}

impl<Chain: cw_orch::environment::CwEnv> abstract_interface::StandaloneDeployer<Chain>
    for OrderbookStandaloneInterface<Chain>
{
}
