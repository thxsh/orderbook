use crate::contract::OrderbookAdapter;

use abstract_adapter::objects::AccountId;
use cosmwasm_schema::QueryResponses;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_adapter::adapter_msg_types!(OrderbookAdapter, OrderbookAdapterExecuteMsg, OrderbookAdapterQueryMsg);

/// Adapter instantiate message
#[cosmwasm_schema::cw_serde]
pub struct OrderbookAdapterInstantiateMsg {}

/// Adapter execute messages
#[cosmwasm_schema::cw_serde]
pub enum OrderbookAdapterExecuteMsg {
    /// Set status of your account
    SetStatus { status: String },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
}

/// Adapter query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum OrderbookAdapterQueryMsg {
    #[returns(StatusResponse)]
    Status { account_id: AccountId },
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}
