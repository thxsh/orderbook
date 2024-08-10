use crate::{
    contract::{Orderbook, OrderbookResult},
    OrderbookError,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Uint128};

#[allow(clippy::too_many_arguments)]
pub fn market_order(
    _deps: DepsMut,
    _env: Env,
    api: Orderbook,
    _info: MessageInfo,
    _base: String,
    _quote: String,
    side: String,
) -> OrderbookResult {
    // validate side
    if &side != "buy" && &side != "sell" {
        return Err(OrderbookError::InvalidSide(side));
    }

    // for buy orders, sort asks by price and quantity

    Ok(api.response("increment"))
}
