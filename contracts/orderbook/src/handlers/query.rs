use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::{BidsResponse, ConfigResponse, OrderbookQueryMsg},
    state::{BIDS, CONFIG},
};

use abstract_app::objects::AssetEntry;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::Bound;

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _module: &Orderbook,
    msg: OrderbookQueryMsg,
) -> OrderbookResult<Binary> {
    match msg {
        OrderbookQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        OrderbookQueryMsg::Bids {} => to_json_binary(&query_bids(deps)?),
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
        .map(|item| item.map(|(k, d)| (AssetEntry::from(k), d)))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(BidsResponse { bids })
}
