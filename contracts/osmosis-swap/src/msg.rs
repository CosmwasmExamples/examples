use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub fee: u64 // 100 = 1%
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Swap {
        pool_id: u64,
        amount_out_min: String,
        denom_out: String,
    },
}

#[cw_serde]
pub struct MigrateMsg {
    pub fee: u64 // 100 = 1%
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // #[returns(YourQueryResponse)]
    // YourQuery {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
