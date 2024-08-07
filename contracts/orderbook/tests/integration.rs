use orderbook::{
    contract::interface::OrderbookInterface,
    msg::{
        BidsResponse, ConfigResponse, OrderbookExecuteMsgFns, OrderbookInstantiateMsg,
        OrderbookQueryMsgFns,
    },
    state::BidAsk,
    OrderbookError, ORDERBOOK_NAMESPACE,
};

use abstract_app::objects::{namespace::Namespace, AssetEntry};
use abstract_client::{AbstractClient, Application, Environment};
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

        // You can set up Abstract with a builder.
        let abs_client = AbstractClient::builder(mock).build()?;
        // The app supports setting balances for addresses and configuring ANS.
        abs_client.set_balance(sender, &coins(123, "ucosm"))?;

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
    let add_balance = coins(100, "osmo");
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
    let mut app = env.app;
    let sender = env.abs.environment().sender_addr();

    // make sure 0 price doesn't work
    let err: OrderbookError = app
        .limit_order(
            AssetEntry::new("OSMO"),
            Decimal::zero(),
            Uint128::one(),
            "buy",
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroPrice);

    // make sure 0 quantity doesn't work
    let err: OrderbookError = app
        .limit_order(
            AssetEntry::new("OSMO"),
            Decimal::one(),
            Uint128::zero(),
            "buy",
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::ZeroQuantity);

    // make sure invalid side doesn't work
    let err: OrderbookError = app
        .limit_order(
            AssetEntry::new("OSMO"),
            Decimal::one(),
            Uint128::one(),
            "invalid",
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookError::InvalidSide("invalid".to_string()));

    // make sure it works
    let _ = app.limit_order(
        AssetEntry::new("OSMO"),
        Decimal::one(),
        Uint128::one(),
        "buy",
    )?;

    let bids: BidsResponse = app.bids()?;
    println!("bids {:#?}", bids);

    assert_eq!(bids.bids.len(), 1);
    assert_eq!(
        bids.bids[0],
        (
            AssetEntry::new("OSMO"),
            vec![BidAsk {
                account: sender,
                price: Decimal::one(),
                quantity: Uint128::one(),
            }]
        )
    );

    Ok(())
}
