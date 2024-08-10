use std::str::FromStr;

use abstract_app::objects::{ans_host::AnsHost, AccountId, AssetEntry};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Endian, IntKey, Item, Key, KeyDeserialize, Map, PrimaryKey};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

#[cosmwasm_schema::cw_serde]
pub struct BidAsk {
    pub account: Addr,
    pub price: Decimal,
    pub quantity: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const LAST_PRICE: Map<(String, String), Uint128> = Map::new("last_price");

// {
//    (base_asset: "uosmo", quote_asset: "atom"): [
//          { account: "addr1", price: 1.0, quantity: 1000 },
//          { account: "addr2", price: 1.1, quantity: 1000 }
//    ]
// }
pub const BIDS: Map<(String, String), Vec<BidAsk>> = Map::new("bids");
pub const ASKS: Map<(String, String), Vec<BidAsk>> = Map::new("asks");
