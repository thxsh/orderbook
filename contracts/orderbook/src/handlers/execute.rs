use crate::{
    contract::{
        Orderbook, OrderbookResult
    },
    msg::OrderbookExecuteMsg,
    state::{CONFIG, COUNT},
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    module: Orderbook,
    msg: OrderbookExecuteMsg,
) -> OrderbookResult {
    match msg {
        OrderbookExecuteMsg::UpdateConfig {} => update_config(deps, info, module),
        OrderbookExecuteMsg::Increment {} => increment(deps, module),
        OrderbookExecuteMsg::Reset { count } => reset(deps, info, count, module),
    }
}

/// Update the configuration of the app
fn update_config(deps: DepsMut, msg_info: MessageInfo, module: Orderbook) -> OrderbookResult {
    // Only the admin should be able to call this
    module.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}

fn increment(deps: DepsMut, module: Orderbook) -> OrderbookResult {
    COUNT.update(deps.storage, |count| OrderbookResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}

fn reset(deps: DepsMut, info: MessageInfo, count: i32, module: Orderbook) -> OrderbookResult {
    module.admin.assert_admin(deps.as_ref(), &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(module.response("reset"))
}
