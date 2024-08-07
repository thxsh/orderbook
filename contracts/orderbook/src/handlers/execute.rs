use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::OrderbookExecuteMsg,
    state::{BidAsk, ASKS, BIDS, CONFIG},
    OrderbookError,
};

use abstract_app::{objects::AssetEntry, traits::AbstractResponse};
use cosmwasm_std::{Addr, Decimal, DepsMut, Env, MessageInfo, Uint128};
use cw_storage_plus::Map;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Orderbook,
    msg: OrderbookExecuteMsg,
) -> OrderbookResult {
    match msg {
        OrderbookExecuteMsg::UpdateConfig {} => update_config(deps, env, info, module),
        OrderbookExecuteMsg::Reset {} => reset(deps, env, info, module),
        OrderbookExecuteMsg::LimitOrder {
            asset,
            price,
            quantity,
            side,
        } => limit_order(deps, env, module, info.sender, asset, price, quantity, side),
        OrderbookExecuteMsg::MarketOrder {
            asset,
            quantity,
            side,
        } => market_order(deps, env, module, asset, quantity, side),
    }
}

/// Update the configuration of the app
fn update_config(
    deps: DepsMut,
    env: Env,
    msg_info: MessageInfo,
    module: Orderbook,
) -> OrderbookResult {
    // Only the admin should be able to call this
    module.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}

fn reset(deps: DepsMut, env: Env, info: MessageInfo, module: Orderbook) -> OrderbookResult {
    module.admin.assert_admin(deps.as_ref(), &info.sender)?;

    Ok(module.response("reset"))
}

pub fn limit_order(
    deps: DepsMut,
    env: Env,
    module: Orderbook,
    account: Addr,
    asset: AssetEntry,
    price: Decimal,
    quantity: Uint128,
    side: String,
) -> OrderbookResult {
    println!(
        "limit_order: account: {:?}, asset: {:?}, price: {:?}, quantity: {:?}, side: {:?}",
        account, asset, price, quantity, side
    );

    // validate side
    if &side != "buy" && &side != "sell" {
        return Err(OrderbookError::InvalidSide(side));
    }

    // validate price
    if price.is_zero() {
        return Err(OrderbookError::ZeroPrice);
    }

    // validate quantity
    if quantity.is_zero() {
        return Err(OrderbookError::ZeroQuantity);
    }

    // for buy orders, place the order in the bids
    // for sell orders, place the order in the asks
    if &side == "buy" {
        // find by price key and push to vector of orders
        let priced_bids = BIDS.may_load(deps.storage, asset.clone().into())?;
        println!("priced_bids: {:?}", priced_bids);

        let bid = BidAsk {
            account,
            price,
            quantity,
        };

        if priced_bids.is_none() {
            BIDS.save(deps.storage, asset.clone().into(), &vec![bid])?;
        } else {
            let mut bids = priced_bids.unwrap_or(vec![]);
            bids.push(bid);
            BIDS.save(deps.storage, asset.into(), &bids)?;
        }
    } else {
        let priced_asks = ASKS.may_load(deps.storage, asset.clone().into())?;
        println!("priced_asks: {:?}", priced_asks);

        let ask = BidAsk {
            account,
            price,
            quantity,
        };

        if priced_asks.is_none() {
            ASKS.save(deps.storage, asset.clone().into(), &vec![ask])?;
        } else {
            let mut asks = priced_asks.unwrap_or(vec![]);
            asks.push(ask);
            ASKS.save(deps.storage, asset.into(), &asks)?;
        }
    }

    Ok(module.response("limit_order"))
}

pub fn market_order(
    deps: DepsMut,
    env: Env,
    module: Orderbook,
    asset: AssetEntry,
    quantity: Uint128,
    side: String,
) -> OrderbookResult {
    // COUNT.update(deps.storage, |count| OrderbookResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}
