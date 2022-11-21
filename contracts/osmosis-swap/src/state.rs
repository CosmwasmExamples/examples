use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SwapMsgReplyState {
    pub sender: Addr,
    pub denom_out: String,
}

pub const FEE: Item<u64> = Item::new("swap_fee");
pub const SWAP_REPLY_STATES: Item<SwapMsgReplyState> = Item::new("swap_reply_states");