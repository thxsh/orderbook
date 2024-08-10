use crate::contract::{Orderbook, OrderbookResult};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(
    _deps: DepsMut,
    _env: Env,
    module: Orderbook,
    _reply: Reply,
) -> OrderbookResult {
    Ok(module.response("instantiate_reply"))
}
