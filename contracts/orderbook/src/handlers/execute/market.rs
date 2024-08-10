use crate::{
    contract::{Orderbook, OrderbookResult},
    OrderbookError,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Uint128};

pub fn market_order(
    _deps: DepsMut,
    _env: Env,
    api: Orderbook,
    _info: MessageInfo,
    _base: String,
    _quote: String,
    quantity: Uint128,
    side: String,
) -> OrderbookResult {
    // validate quantity
    if quantity.is_zero() {
        return Err(OrderbookError::ZeroQuantity);
    }

    // validate side
    if &side != "buy" && &side != "sell" {
        return Err(OrderbookError::InvalidSide(side));
    }

    // for buy orders, sort asks by price and quantity

    Ok(api.response("increment"))
}
