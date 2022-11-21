use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid Funds Request")]
    InvalidFundsRequest {},

    #[error("Failed To Add Liquidity: {reason:?}")]
    FailAddLiquidity { reason: String },

    #[error("Failed To Remove Liquidity: {reason:?}")]
    FailRemoveLiquidity { reason: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
