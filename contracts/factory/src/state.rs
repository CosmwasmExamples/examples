use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Name {
    pub name: String,
    pub contract: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateReplyState {
    pub name: String,
    pub owner: Addr,
}
pub const NAMES: Map<&Addr, Name> = Map::new("names");
pub const CHILD_CONTRACT_CODE_ID: Item<u64> = Item::new("code_id");
pub const REPLY_STORAGE: Map<u64, InstantiateReplyState> = Map::new("reply_storage");