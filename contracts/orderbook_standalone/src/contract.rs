use abstract_standalone::sdk::{AbstractResponse, AbstractSdkError, IbcInterface};
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, StdResult};

use crate::{
    msg::{
        ConfigResponse, CountResponse, OrderbookStandaloneExecuteMsg, OrderbookStandaloneInstantiateMsg,
        OrderbookStandaloneMigrateMsg, OrderbookStandaloneQueryMsg,
    },
    state::{Config, CONFIG, COUNT},
    OrderbookStandalone, OrderbookStandaloneResult, ORDERBOOK_STANDALONE, ORDERBOOK_STANDALONE_ID,
};

const INSTANTIATE_REPLY_ID: u64 = 0;

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OrderbookStandaloneInstantiateMsg,
) -> OrderbookStandaloneResult {
    let config: Config = Config {};
    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &msg.count)?;

    // Init standalone as module
    let is_migratable = true;
    ORDERBOOK_STANDALONE.instantiate(deps.branch(), info, msg.base, is_migratable)?;

    Ok(ORDERBOOK_STANDALONE.response("init"))
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OrderbookStandaloneExecuteMsg,
) -> OrderbookStandaloneResult {
    let standalone = ORDERBOOK_STANDALONE;
    match msg {
        OrderbookStandaloneExecuteMsg::UpdateConfig {} => update_config(deps, info, standalone),
        OrderbookStandaloneExecuteMsg::Increment {} => increment(deps, standalone),
        OrderbookStandaloneExecuteMsg::Reset { count } => reset(deps, info, count, standalone),
        OrderbookStandaloneExecuteMsg::IbcCallback(msg) => {
            let ibc_client = ORDERBOOK_STANDALONE.ibc_client(deps.as_ref());

            let ibc_client_addr = ibc_client.module_address()?;
            if info.sender.ne(&ibc_client_addr) {
                return Err(AbstractSdkError::CallbackNotCalledByIbcClient {
                    caller: info.sender,
                    client_addr: ibc_client_addr,
                    module: ORDERBOOK_STANDALONE_ID.to_owned(),
                }
                .into());
            };
            // Parse msg.callback here!
            Ok(ORDERBOOK_STANDALONE
                .response("test_ibc")
                .set_data(msg.callback.msg))
        }
        OrderbookStandaloneExecuteMsg::ModuleIbc(_msg) => {
            todo!()
        }
    }
}

/// Update the configuration of the standalone
fn update_config(deps: DepsMut, info: MessageInfo, module: OrderbookStandalone) -> OrderbookStandaloneResult {
    ORDERBOOK_STANDALONE
        .admin
        .assert_admin(deps.as_ref(), &info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}

fn increment(deps: DepsMut, module: OrderbookStandalone) -> OrderbookStandaloneResult {
    COUNT.update(deps.storage, |count| OrderbookStandaloneResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}

fn reset(
    deps: DepsMut,
    info: MessageInfo,
    count: i32,
    module: OrderbookStandalone,
) -> OrderbookStandaloneResult {
    ORDERBOOK_STANDALONE
        .admin
        .assert_admin(deps.as_ref(), &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(module.response("reset"))
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn query(deps: Deps, _env: Env, msg: OrderbookStandaloneQueryMsg) -> StdResult<Binary> {
    let _standalone = &ORDERBOOK_STANDALONE;
    match msg {
        OrderbookStandaloneQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        OrderbookStandaloneQueryMsg::Count {} => to_json_binary(&query_count(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let count = COUNT.load(deps.storage)?;
    Ok(CountResponse { count })
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> OrderbookStandaloneResult {
    match msg.id {
        self::INSTANTIATE_REPLY_ID => Ok(crate::ORDERBOOK_STANDALONE.response("instantiate_reply")),
        _ => todo!(),
    }
}

/// Handle the standalone migrate msg
#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: OrderbookStandaloneMigrateMsg) -> OrderbookStandaloneResult {
    // The Abstract Standalone object does version checking and
    ORDERBOOK_STANDALONE.migrate(deps)?;
    Ok(ORDERBOOK_STANDALONE.response("migrate"))
}
