use std::str::FromStr;

use abstract_interface::ExecuteMsgFns;

use cw_asset::AssetInfoBase;
use orderbook::{
    contract::interface::OrderbookInterface,
    msg::{
        AsksResponse, BidsResponse, ConfigResponse, OrderbookExecuteMsgFns,
        OrderbookInstantiateMsg, OrderbookQueryMsgFns,
    },
    state::BidAsk,
    OrderbookError, ORDERBOOK_NAMESPACE,
};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{
    builder::cw20_builder::Cw20QueryMsgFns, AbstractClient, Application, Environment,
};
use cosmwasm_std::{coins, Decimal, Uint128};
use cw_controllers::AdminError;
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    abs: AbstractClient<Env>,
    app: Application<Env, OrderbookInterface<Env>>,
}

impl TestEnv<MockBech32> {
    /// Set up the test environment with an Account that has the App installed
    fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
        // Create a sender and mock env
        let mock = MockBech32::new("mock");
        let sender = mock.sender_addr();
        let namespace = Namespace::new(ORDERBOOK_NAMESPACE)?;

        // set up Abstract with a builder.
        let abs_client = AbstractClient::builder(mock.clone()).build()?;

        // add balance of all tokens to the sender
        abs_client.add_balance(sender.clone(), &coins(1000, "atom"))?;
        abs_client.add_balance(sender.clone(), &coins(1000, "uosmo"))?;
        abs_client.add_balance(sender.clone(), &coins(1000, "ntrn"))?;
        abs_client.add_balance(sender.clone(), &coins(1000, "juno"))?;

        // register tokens with ANS
        let atom = AssetInfoBase::native("atom");
        let uosmo = AssetInfoBase::native("uosmo");
        let ntrn = AssetInfoBase::native("ntrn");
        let juno = AssetInfoBase::native("juno");

        let ans = abs_client.name_service();
        ans.update_asset_addresses(
            vec![
                ("atom".into(), atom.clone()),
                ("uosmo".into(), uosmo.clone()),
                ("ntrn".into(), ntrn.clone()),
                ("juno".into(), juno.clone()),
            ],
            vec![],
        )?;

        // mint some cw20 tokens to the sender
        // let cw20 = Cw20Base::new("cw20", mock.clone());
        // cw20.upload()?;
        // let resp = cw20.instantiate(
        //     &cw20_base::InstantiateMsg {
        //         name: "Test".to_string(),
        //         symbol: "TEST".to_string(),
        //         decimals: 6,
        //         initial_balances: vec![],
        //         mint: Some(MinterResponse {
        //             minter: sender.to_string(),
        //             cap: None,
        //         }),
        //         marketing: None,
        //     },
        //     None,
        //     None,
        // )?;
        // let cw20_address = resp.event_attr_value("instantiate", "_contract_address")?;
        // println!("cw20 address: {:#?}", cw20_address);
        // cw20.mint(1_000_000_u128, sender.to_string())?;
        // let balance = cw20.balance(sender.clone())?;
        // println!("balance: {:#?}", balance);

        // get native balance
        let native_balance = abs_client.environment().query_all_balances(&sender)?;
        assert!(native_balance.iter().any(|coin| coin.denom == "atom"));
        assert!(native_balance.iter().any(|coin| coin.denom == "uosmo"));
        assert!(native_balance.iter().any(|coin| coin.denom == "ntrn"));
        assert!(native_balance.iter().any(|coin| coin.denom == "juno"));

        // Publish the app
        let publisher = abs_client.publisher_builder(namespace).build()?;
        publisher.publish_app::<OrderbookInterface<_>>()?;

        let app = publisher
            .account()
            .install_app::<OrderbookInterface<_>>(&OrderbookInstantiateMsg {}, &[])?;

        Ok(TestEnv {
            abs: abs_client,
            app,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let config = app.config()?;
    assert_eq!(config, ConfigResponse {});
    Ok(())
}

#[test]
fn successful_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    app.reset()?;
    let bids: BidsResponse = app.bids()?;
    assert_eq!(bids.bids.len(), 0);
    Ok(())
}

#[test]
fn failed_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let err: OrderbookError = app
        .call_as(&Addr::unchecked("NotAdmin"))
        .reset()
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::Admin(AdminError::NotAdmin {}));
    Ok(())
}

#[test]
fn update_config() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    app.update_config()?;
    let config = app.config()?;
    let expected_response = orderbook::msg::ConfigResponse {};
    assert_eq!(config, expected_response);
    Ok(())
}

#[test]
fn balance_added() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account = env.app.account();

    // You can add balance to your account in test environment
    let add_balance = coins(100, "balance_test");
    account.add_balance(&add_balance)?;
    let balances = account.query_balances()?;

    assert_eq!(balances, add_balance);

    // Or set balance to any other address using cw_orch
    let mock_env = env.abs.environment();
    mock_env.add_balance(&env.app.address()?, add_balance.clone())?;
    let balances = mock_env.query_all_balances(&env.app.address()?)?;

    assert_eq!(balances, add_balance);
    Ok(())
}

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
            Uint128::one(),
            atom_asset.clone(),
            "buy",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroPrice);

    // make sure 0 quantity doesn't work
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            Uint128::zero(),
            atom_asset.clone(),
            "buy",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroQuantity);

    // make sure invalid side doesn't work
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            Uint128::one(),
            atom_asset.clone(),
            "invalid",
            &atom_coins,
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidSide("invalid".to_string()));

    // make sure the deposited quantity matches the quantity in the order
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            Uint128::one(),
            atom_asset.clone(),
            "buy",
            &coins(2, "atom"),
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidQuantity);

    // make sure the deposited asset matches the asset expected by the side of the order
    let err: OrderbookError = app
        .limit_order(
            osmo_asset.clone(),
            Decimal::one(),
            Uint128::one(),
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
        Uint128::one(),
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
        Uint128::one(),
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
        Uint128::one(),
        atom_asset.clone(),
        "sell",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("4.0")?,
        Uint128::one(),
        atom_asset.clone(),
        "sell",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("2.0")?,
        Uint128::one(),
        atom_asset.clone(),
        "buy",
        &atom_coins,
    )?;
    app.limit_order(
        osmo_asset.clone(),
        Decimal::from_str("1.0")?,
        Uint128::one(),
        atom_asset.clone(),
        "buy",
        &atom_coins,
    )?;

    // make sure 0 quantity doesn't work
    let err: OrderbookError = app
        .market_order(
            osmo_asset.clone(),
            Uint128::zero(),
            atom_asset.clone(),
            "buy",
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroQuantity);

    // make sure invalid side doesn't work
    let err: OrderbookError = app
        .market_order(
            osmo_asset.clone(),
            Uint128::one(),
            atom_asset.clone(),
            "invalid",
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidSide("invalid".to_string()));

    // make sure it works
    let _ = app.market_order(
        osmo_asset.clone(),
        Uint128::one(),
        atom_asset.clone(),
        "buy",
    )?;

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
