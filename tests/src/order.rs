use std::str::FromStr;

use orderbook::{
    msg::{AsksResponse, BidsResponse, OrderbookExecuteMsgFns, OrderbookQueryMsgFns},
    state::BidAsk,
    OrderbookError,
};

use abstract_client::Environment;
use cosmwasm_std::{coins, Decimal, Uint128};

// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

use super::common::TestEnv;

#[test]
fn place_limit_order() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let abs = env.abs;
    let sender = abs.environment().sender_addr();

    let balances = abs.environment().query_all_balances(&sender)?;
    let atom_balance_before = balances
        .iter()
        .find(|coin| coin.denom == "atom")
        .unwrap()
        .amount;
    let osmo_balance_before = balances
        .iter()
        .find(|coin| coin.denom == "uosmo")
        .unwrap()
        .amount;

    let osmo_asset = "uosmo".to_string();
    let atom_asset = "atom".to_string();
    let _ntrn_asset = "ntrn".to_string();
    let _juno_asset = "juno".to_string();

    let atom_coins = coins(1, "atom");
    let osmo_coins = coins(1, "uosmo");

    // make sure 0 price doesn't work
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::zero(),
            atom_asset.clone(),
            "buy",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroPrice);

    // make sure 0 quantity doesn't work
    let err: cw_orch::anyhow::Error = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            atom_asset.clone(),
            "buy",
            &coins(0, "atom"),
        )
        .unwrap_err()
        .downcast::<OrderbookError>()
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        "Cannot transfer empty coins amount".to_string()
    );

    // make sure invalid side doesn't work
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            atom_asset.clone(),
            "invalid",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidSide("invalid".to_string()));

    // make sure the deposited asset matches the asset expected by the side of the order
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            atom_asset.clone(),
            "sell",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::IncorrectAsset);

    // make sure bids work
    let _ = app.limit_order(
        osmo_asset.clone(),
        Decimal::one(),
        atom_asset.clone(),
        "buy",
        &atom_coins,
    )?;

    // make sure the atom asset (quote) is reserved from the sender for buy orders
    let balances = abs.environment().balance(&sender, Some("atom".into()))?;
    // println!("balances: {:#?}", balances);
    assert_eq!(balances, coins(atom_balance_before.u128() - 1, "atom"));

    // make sure atom asset is held at the contract abstract account proxy
    let icaa_address = app.account().proxy()?;
    assert_eq!(
        &abs.environment()
            .balance(icaa_address, Some("atom".into()))?,
        &atom_coins,
    );

    let bids_resp: BidsResponse = app.bids()?;
    println!("bids {:#?}", bids_resp);

    assert_eq!(bids_resp.bids.len(), 1);
    assert_eq!(
        bids_resp.bids[0],
        (
            (osmo_asset.clone(), atom_asset.clone(),),
            vec![BidAsk {
                account: sender.clone(),
                price: Decimal::one(),
                quantity: Uint128::one(),
            }]
        )
    );

    // make sure asks work
    let _ = app.limit_order(
        osmo_asset.clone(),
        Decimal::one(),
        atom_asset.clone(),
        "sell",
        &osmo_coins,
    )?;

    // make sure the osmo asset (base) is reserved from the sender for sell orders
    let balances = abs.environment().balance(&sender, Some("uosmo".into()))?;
    assert_eq!(balances, coins(osmo_balance_before.u128() - 1, "uosmo"));

    // make sure osmo asset is held at the contract abstract account proxy
    let icaa_address = app.account().proxy()?;
    assert_eq!(
        &abs.environment()
            .balance(icaa_address, Some("uosmo".into()))?,
        &coins(1, "uosmo"),
    );

    let asks_resp: AsksResponse = app.asks()?;
    println!("asks {:#?}", asks_resp);

    assert_eq!(asks_resp.asks.len(), 1);
    assert_eq!(
        asks_resp.asks[0],
        (
            (osmo_asset.clone(), atom_asset.clone(),),
            vec![BidAsk {
                account: sender.clone(),
                price: Decimal::one(),
                quantity: Uint128::one(),
            }]
        )
    );

    Ok(())
}

#[test]
#[ignore]
fn place_market_order() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;
    let _sender = env.abs.environment().sender_addr();

    let osmo_asset = "uosmo".to_string();
    let atom_asset = "atom".to_string();
    let _ntrn_asset = "ntrn".to_string();
    let _juno_asset = "juno".to_string();

    let atom_coins = coins(1, "atom");

    // add some limit orders
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("3.0")?,
        atom_asset.clone(),
        "sell",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("4.0")?,
        atom_asset.clone(),
        "sell",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("2.0")?,
        atom_asset.clone(),
        "buy",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("1.0")?,
        atom_asset.clone(),
        "buy",
        &atom_coins,
    )?;

    // make sure 0 quantity doesn't work
    let err: OrderbookError = app
        .market_order(
            osmo_asset.clone(),
            atom_asset.clone(),
            "buy",
            &coins(0, "atom"),
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroQuantity);

    // make sure invalid side doesn't work
    let err: OrderbookError = app
        .market_order(
            osmo_asset.clone(),
            atom_asset.clone(),
            "invalid",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidSide("invalid".to_string()));

    // make sure it works
    let _ = app.market_order(osmo_asset.clone(), atom_asset.clone(), "buy", &atom_coins)?;

    // // make sure balance of sender was updated
    // let balances = app.account().query_balances()?;
    // assert_eq!(balances, coins(1, "osmo"));

    // let bids: BidsResponse = app.bids()?;
    // println!("bids {:#?}", bids);

    // assert_eq!(bids.bids.len(), 1);
    // assert_eq!(
    //     bids.bids[0],
    //     (
    //         asset.clone(),
    //         vec![BidAsk {
    //             account: sender,
    //             price: Decimal::zero(),
    //             quantity: Uint128::one(),
    //         }]
    //     )
    // );

    Ok(())
}
