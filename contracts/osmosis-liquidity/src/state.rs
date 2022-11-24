use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AddLiquidityState {
    pub sender: Addr,
    pub pool_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RemoveLiquidityState {
  pub sender: Addr,
  pub pool_id: u64,
  pub denom_out: String,
}

pub const ADD_LIQUIDITY_STATE: Item<AddLiquidityState> = Item::new("add_liquidity_state");
pub const REMOVE_LIQUIDITY_STATE: Item<RemoveLiquidityState> = Item::new("remove_liquidity_state");
