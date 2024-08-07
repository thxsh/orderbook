use crate::{contract::Orderbook, state::BidAsk};

use abstract_app::objects::AssetEntry;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Decimal, Uint128};

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
    LimitOrder {
        asset: AssetEntry,
        price: Decimal,
        quantity: Uint128,
        side: String, // "buy" or "sell"
    },
    // Place a market order
    MarketOrder {
        asset: AssetEntry,
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
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct BidsResponse {
    pub bids: Vec<(AssetEntry, Vec<BidAsk>)>,
}
