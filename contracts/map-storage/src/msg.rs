use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Set {
        key: String,
        value: String,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<(String,String)>)]
    GetMap {},
    #[returns(Vec<String>)]
    AllKeys {},
}