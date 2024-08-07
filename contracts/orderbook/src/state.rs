use std::str::FromStr;

use abstract_app::objects::{AccountId, AssetEntry};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Endian, IntKey, Item, Key, KeyDeserialize, Map, PrimaryKey};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

#[cosmwasm_schema::cw_serde]
#[derive(Default)]
pub struct AssetId(pub String);

impl From<AssetEntry> for AssetId {
    fn from(asset: AssetEntry) -> Self {
        AssetId(asset.to_string())
    }
}

impl<'a> PrimaryKey<'a> for AssetId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        self.0.key()
    }
}

impl KeyDeserialize for AssetId {
    type Output = String;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        String::from_vec(value)
    }
}

#[cosmwasm_schema::cw_serde]
pub struct BidAsk {
    pub account: Addr,
    pub price: Decimal,
    pub quantity: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const LAST_PRICE: Map<AssetId, Uint128> = Map::new("last_price");
pub const BIDS: Map<AssetId, Vec<BidAsk>> = Map::new("bids");
pub const ASKS: Map<AssetId, Vec<BidAsk>> = Map::new("asks");
