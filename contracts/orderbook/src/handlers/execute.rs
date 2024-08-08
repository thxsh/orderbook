use crate::{
    contract::{Orderbook, OrderbookResult},
    msg::OrderbookExecuteMsg,
    state::{BidAsk, ASKS, BIDS, CONFIG},
    OrderbookError,
};

use abstract_app::{
    objects::AssetEntry,
    sdk::{AccountVerification, AccountingInterface, Resolve, TransferInterface},
    traits::{AbstractNameService, AbstractResponse},
};
use cosmwasm_std::{to_json_binary, Addr, Decimal, DepsMut, Env, MessageInfo, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
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
        let bid = BidAsk {
            account: account.clone(),
            price,
            quantity,
        };

        // let balance = bank.balance(&asset)?;
        // println!("balance: {:?}", balance);
        let bank = module.bank(deps.as_ref());
        let vault = module.accountant(deps.as_ref());
        let list = vault.assets_list()?;
        println!("list: {:?}", list);

        
        // let balnc = bank.balance(&asset)?;
        // println!("balnc: {:?}", balnc);

        let ans = module.name_service(deps.as_ref());
        let asset_info = ans.query(&asset);
        println!("asset_info: {:?}", asset_info);
        // let asset_info =
        // reserve the cw asset from the sender
        // Check if the sender has enough balance and send tokens to the contract
        // let transfer_cw20 = WasmMsg::Execute {
        //     contract_addr: account.clone().to_string(),
        //     msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
        //         owner: account.clone().to_string(),
        //         recipient: env.contract.address.to_string(),
        //         amount: Uint128::one(),
        //     })?,
        //     funds: vec![],
        // };

        // find by price key and push to vector of orders
        let priced_bids = BIDS.may_load(deps.storage, asset.clone().into())?;
        println!("priced_bids: {:?}", priced_bids);

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
    // validate quantity
    if quantity.is_zero() {
        return Err(OrderbookError::ZeroQuantity);
    }

    // validate side
    if &side != "buy" && &side != "sell" {
        return Err(OrderbookError::InvalidSide(side));
    }

    // for buy orders, sort asks by price and quantity

    Ok(module.response("increment"))
}
