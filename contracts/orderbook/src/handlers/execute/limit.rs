use crate::{
    contract::{Orderbook, OrderbookResult},
    state::{BidAsk, ASKS, BIDS},
    OrderbookError,
};

use abstract_app::{
    objects::AssetEntry,
    sdk::TransferInterface,
    traits::{AbstractNameService, AbstractResponse},
};
use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Uint128};

fn verify_deposit(info: MessageInfo, denom: &str) -> OrderbookResult<Uint128> {
    if let Some(funds) = info.funds.iter().find(|coin| coin.denom == denom) {
        println!("funds: {:?}", funds);
        if funds.amount.is_zero() {
            return Err(OrderbookError::ZeroQuantity);
        }

        Ok(funds.amount)
    } else {
        // TODO >> return the funds back to the sender
        Err(OrderbookError::IncorrectAsset)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn limit_order(
    deps: DepsMut,
    _env: Env,
    api: Orderbook,
    info: MessageInfo,
    base: String,
    quote: String,
    price: Decimal,
    side: String,
) -> OrderbookResult {
    let sender = info.sender.clone();

    // println!(
    //     "limit_order: sender: {:?}, base: {:?}, quote: {:?}, price: {:?}, side: {:?}",
    //     sender, base, quote, price, side
    // );

    // validate side
    if &side != "buy" && &side != "sell" {
        return Err(OrderbookError::InvalidSide(side));
    }

    // validate price
    if price.is_zero() {
        return Err(OrderbookError::ZeroPrice);
    }

    let bank = api.bank(deps.as_ref());

    let ans = api.name_service(deps.as_ref());
    let _base_asset = ans.query(&AssetEntry::new(&base))?;
    let _quote_asset = ans.query(&AssetEntry::new(&quote))?;
    // println!("base_asset: {:?}", base_asset);
    // println!("quote_asset: {:?}", quote_asset);

    let market = (base.clone(), quote.clone());

    // for buy orders, place the order in the bids using quote_asset
    // for sell orders, place the order in the asks using base_asset
    let deposit = if &side == "buy" {
        // make sure quote_asset funds deposited
        let quantity = verify_deposit(info.clone(), quote.as_str())?;

        let deposit = bank.deposit(info.funds)?;

        let bid = BidAsk {
            account: sender.clone(),
            price,
            quantity,
        };

        // find by market pair key and push to vector of orders
        let bids_by_market = BIDS.may_load(deps.storage, market.clone())?;
        println!("bids_by_market: {:?}", bids_by_market);

        if bids_by_market.is_none() {
            BIDS.save(deps.storage, market.clone(), &vec![bid])?;
        } else {
            let mut bids = bids_by_market.unwrap_or(vec![]);
            bids.push(bid);
            BIDS.save(deps.storage, market.clone(), &bids)?;
        }

        deposit
    } else {
        // make sure base_asset funds deposited
        let quantity = verify_deposit(info.clone(), base.as_str())?;

        let deposit = bank.deposit(info.funds)?;

        let ask = BidAsk {
            account: sender.clone(),
            price,
            quantity,
        };

        // find by market pair key and push to vector of orders
        let asks_by_market = ASKS.may_load(deps.storage, market.clone())?;
        println!("asks_by_market: {:?}", asks_by_market);

        if asks_by_market.is_none() {
            ASKS.save(deps.storage, market.clone(), &vec![ask])?;
        } else {
            let mut asks = asks_by_market.unwrap_or(vec![]);
            asks.push(ask);
            ASKS.save(deps.storage, market.clone(), &asks)?;
        }

        deposit
    };

    Ok(api.response("limit_order").add_messages(deposit))
}
