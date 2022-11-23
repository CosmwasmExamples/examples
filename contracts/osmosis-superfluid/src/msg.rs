use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub validator_address: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    LockAndDelegate {},
    UndelegateAndUnbond {
        lock_id: u64,
    },
    Withdraw {
        amount: String,
        denom: String,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

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
