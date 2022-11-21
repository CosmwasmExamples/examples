use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AddLiquidity {
        pool_id: u64,
        min_shares: String
    },
    RemoveLiquidity {
        pool_id: u64,
        denom_out: String,
        min_tokens_out: String,
    }
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
