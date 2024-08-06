use cw_controllers::AdminError;
use orderbook_standalone::{
    msg::{
        ConfigResponse, CountResponse, OrderbookStandaloneExecuteMsgFns, OrderbookStandaloneInstantiateMsg,
        OrderbookStandaloneQueryMsgFns,
    },
    OrderbookStandaloneError, OrderbookStandaloneInterface, ORDERBOOK_NAMESPACE,
};

use abstract_client::{AbstractClient, Application, Environment};
use abstract_standalone::{objects::namespace::Namespace, std::standalone};
use cosmwasm_std::coins;
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    abs: AbstractClient<Env>,
    standalone: Application<Env, OrderbookStandaloneInterface<Env>>,
}

impl TestEnv<MockBech32> {
    /// Set up the test environment with an Account that has the Standalone installed
    fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
        // Create a sender and mock env
        let mock = MockBech32::new("mock");
        let sender = mock.sender_addr();
        let namespace = Namespace::new(ORDERBOOK_NAMESPACE)?;

        // You can set up Abstract with a builder.
        let abs_client = AbstractClient::builder(mock).build()?;
        // The standalone supports setting balances for addresses and configuring ANS.
        abs_client.set_balance(sender.clone(), &coins(123, "ucosm"))?;

        // Publish the standalone
        let publisher = abs_client.publisher_builder(namespace).build()?;
        publisher.publish_standalone::<OrderbookStandaloneInterface<_>>()?;

        let standalone = publisher
            .account()
            .install_standalone::<OrderbookStandaloneInterface<_>>(
                &OrderbookStandaloneInstantiateMsg {
                    base: standalone::StandaloneInstantiateMsg {
                        ans_host_address: abs_client.name_service().addr_str()?,
                        version_control_address: abs_client.version_control().addr_str()?,
                    },
                    count: 0,
                },
                &[],
            )?;

        Ok(TestEnv {
            abs: abs_client,
            standalone,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let standalone = env.standalone;

    let config = standalone.config()?;
    assert_eq!(config, ConfigResponse {});
    Ok(())
}

#[test]
fn successful_increment() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let standalone = env.standalone;

    standalone.increment()?;
    let count: CountResponse = standalone.count()?;
    assert_eq!(count.count, 1);
    Ok(())
}

#[test]
fn successful_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let standalone = env.standalone;

    standalone.reset(42)?;
    let count: CountResponse = standalone.count()?;
    assert_eq!(count.count, 42);
    Ok(())
}

#[test]
fn update_config() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let standalone = env.standalone;

    standalone.update_config()?;
    let config = standalone.config()?;
    let expected_response = orderbook_standalone::msg::ConfigResponse {};
    assert_eq!(config, expected_response);
    Ok(())
}

#[test]
fn balance_added() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account = env.standalone.account();

    // You can add balance to your account in test environment
    let add_balance = coins(100, "ucosm");
    account.add_balance(&add_balance)?;
    let balances = account.query_balances()?;

    assert_eq!(balances, add_balance);

    // Or set balance to any other address using cw_orch
    let mock_env = env.abs.environment();
    mock_env.add_balance(&env.standalone.address()?, add_balance.clone())?;
    let balances = mock_env.query_all_balances(&env.standalone.address()?)?;

    assert_eq!(balances, add_balance);
    Ok(())
}

#[test]
fn failed_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let standalone = env.standalone;

    let err: OrderbookStandaloneError = standalone
        .call_as(&Addr::unchecked("NotAdmin"))
        .reset(9)
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, OrderbookStandaloneError::Admin(AdminError::NotAdmin {}));
    Ok(())
}
