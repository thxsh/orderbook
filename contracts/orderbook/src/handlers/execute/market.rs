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

pub fn market_order(
    deps: DepsMut,
    env: Env,
    api: Orderbook,
    info: MessageInfo,
    base: String,
    quote: String,
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
