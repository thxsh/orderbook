use crate::{contract::Orderbook, state::BidAsk};

use abstract_app::objects::account::AccountTrace;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Decimal;

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
        side: String, // "buy" or "sell"
    },
    // Place a market order
    #[cw_orch(payable)]
    MarketOrder {
        base: String,
        quote: String,
        side: String, // "buy" or "sell"
    },
    /// Admin method - reset count
    Reset {},
}

pub type Route = AccountTrace;

#[cosmwasm_schema::cw_serde]
pub struct OrderbookEvent {
    pub base: String,
    pub quote: String,
    pub price: Decimal,
    pub side: String,
    pub quantity: Uint128,
    pub sender: String,
    pub order_type: String,
}

#[cosmwasm_schema::cw_serde]
pub struct Header {
    pub current_hop: u32,
    pub route: Route,
}

#[non_exhaustive]
#[cosmwasm_schema::cw_serde]
pub enum OrderbookIbcMessage {
    /// Route a message
    RouteMessage {
        event: OrderbookEvent,
        header: Header,
    },
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
