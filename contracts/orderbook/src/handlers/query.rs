use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::{AsksResponse, BidsResponse, ConfigResponse, OrderbookQueryMsg},
    state::{ASKS, BIDS, CONFIG},
};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, Order, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _module: &Orderbook,
    msg: OrderbookQueryMsg,
) -> OrderbookResult<Binary> {
    match msg {
        OrderbookQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        OrderbookQueryMsg::Bids {} => to_json_binary(&query_bids(deps)?),
        OrderbookQueryMsg::Asks {} => to_json_binary(&query_asks(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_bids(deps: Deps) -> StdResult<BidsResponse> {
    let bids = BIDS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(BidsResponse { bids })
}

fn query_asks(deps: Deps) -> StdResult<AsksResponse> {
    let asks = ASKS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AsksResponse { asks })
}
