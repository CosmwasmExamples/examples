use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    owner: Addr,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Lock {
        duration: u64,
    },
    Unlock {
        lock_id: u64,
    },
    Withdraw {
        amount: String,
        denom: String,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {
    pub owner: Addr,
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetLockResponse)]
    GetLocks {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetLockResponse {
    pub locks: Vec<u64>,
}
