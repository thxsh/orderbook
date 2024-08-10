use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::OrderbookMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

/// Handle the app migrate msg
/// The top-level Abstract app does version checking and dispatches to this handler
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    module: Orderbook,
    _msg: OrderbookMigrateMsg,
) -> OrderbookResult {
    Ok(module.response("migrate"))
}
