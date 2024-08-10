use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::OrderbookExecuteMsg,
    state::{BidAsk, ASKS, BIDS, CONFIG},
    OrderbookError,
};

use abstract_app::{
    objects::AssetEntry,
    sdk::{base, AccountVerification, AccountingInterface, Resolve, TransferInterface},
    traits::{AbstractNameService, AbstractResponse},
};
use cosmwasm_std::{
    coins, to_json_binary, Addr, Decimal, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use cw_asset::{Asset, AssetBase, AssetInfo};
use cw_storage_plus::Map;
use cw_utils::one_coin;

mod limit;
mod market;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    api: Orderbook,
    msg: OrderbookExecuteMsg,
) -> OrderbookResult {
    match msg {
        OrderbookExecuteMsg::UpdateConfig {} => update_config(deps, env, info, api),
        OrderbookExecuteMsg::Reset {} => reset(deps, env, info, api),
        OrderbookExecuteMsg::LimitOrder {
            base,
            quote,
            price,
            quantity,
            side,
        } => limit::limit_order(deps, env, api, info, base, quote, price, quantity, side),
        OrderbookExecuteMsg::MarketOrder {
            base,
            quote,
            quantity,
            side,
        } => market::market_order(deps, env, api, info, base, quote, quantity, side),
    }
}

/// Update the configuration of the app
fn update_config(
    deps: DepsMut,
    env: Env,
    msg_info: MessageInfo,
    api: Orderbook,
) -> OrderbookResult {
    // Only the admin should be able to call this
    api.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(api.response("update_config"))
}

fn reset(deps: DepsMut, env: Env, info: MessageInfo, api: Orderbook) -> OrderbookResult {
    api.admin.assert_admin(deps.as_ref(), &info.sender)?;

    Ok(api.response("reset"))
}
