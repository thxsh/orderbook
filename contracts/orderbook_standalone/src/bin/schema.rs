use cosmwasm_schema::{remove_schemas, write_api};
use orderbook_standalone::msg::{
    OrderbookStandaloneExecuteMsg, OrderbookStandaloneInstantiateMsg,
    OrderbookStandaloneMigrateMsg, OrderbookStandaloneQueryMsg,
};
use std::env::current_dir;
use std::fs::create_dir_all;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    write_api! {
        name: "schema",
        instantiate: OrderbookStandaloneInstantiateMsg,
        query: OrderbookStandaloneQueryMsg,
        execute: OrderbookStandaloneExecuteMsg,
        migrate: OrderbookStandaloneMigrateMsg,
    };
}
