use crate::{contract::Orderbook, state::BidAsk};

use abstract_app::objects::{ans_host::AnsHost, AssetEntry};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_asset::Asset;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(Orderbook, OrderbookExecuteMsg, OrderbookQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct OrderbookInstantiateMsg {}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum OrderbookExecuteMsg {
    UpdateConfig {},
    /// Place a limit order
    #[cw_orch(payable)]
    LimitOrder {
        base: String,
        quote: String,
        price: Decimal,
        quantity: Uint128,
        side: String, // "buy" or "sell"
    },
    // Place a market order
    MarketOrder {
        base: String,
        quote: String,
        quantity: Uint128,
        side: String, // "buy" or "sell"
    },
    /// Admin method - reset count
    Reset {},
}

#[cosmwasm_schema::cw_serde]
pub struct OrderbookMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum OrderbookQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(BidsResponse)]
    Bids {},
    #[returns(AsksResponse)]
    Asks {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct BidsResponse {
    pub bids: Vec<((String, String), Vec<BidAsk>)>,
}

#[cosmwasm_schema::cw_serde]
pub struct AsksResponse {
    pub asks: Vec<((String, String), Vec<BidAsk>)>,
}
