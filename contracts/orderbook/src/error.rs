use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::StdError;
use cw_asset::AssetError;
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OrderbookError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    DappError(#[from] AppError),

    #[error("{0}")]
    Payment(#[from] cw_utils::PaymentError),

    #[error("Invalid side {0}")]
    InvalidSide(String),

    #[error("Quantity must be greater than zero")]
    ZeroQuantity,

    #[error("Price must be greater than zero")]
    ZeroPrice,

    #[error("Asset deposited does not match the market side")]
    IncorrectAsset,
}
