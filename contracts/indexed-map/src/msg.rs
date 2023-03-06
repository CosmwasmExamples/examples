use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr};
use crate::state::UserRecord;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Set {
        id: u64,
        amount: Uint128,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub struct MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserRecord)]
    Get { id: u64 },
    #[returns(Vec<UserRecord>)]
    GetByOwner { owner: Addr },
}

