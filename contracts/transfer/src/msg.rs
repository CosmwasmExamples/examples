use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    Withdraw {
        amount: String,
        denom: String,
    },
    WithdrawAll {},
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Coin>)]
    GetBalance {},
}
