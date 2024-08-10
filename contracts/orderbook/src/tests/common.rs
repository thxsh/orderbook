use abstract_interface::ExecuteMsgFns;

use cw_asset::AssetInfoBase;
use orderbook::{
    contract::interface::OrderbookInterface, msg::OrderbookInstantiateMsg, ORDERBOOK_NAMESPACE,
};

use abstract_app::objects::namespace::Namespace;
use abstract_client::{AbstractClient, Application, Environment};
use cosmwasm_std::coins;

// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

pub struct TestEnv<Env: CwEnv> {
    pub abs: AbstractClient<Env>,
    pub app: Application<Env, OrderbookInterface<Env>>,
}

impl TestEnv<MockBech32> {
    /// Set up the test environment with an Account that has the App installed
    pub fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
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
