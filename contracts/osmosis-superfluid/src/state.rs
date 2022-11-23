use cosmwasm_std::Addr;
use cw_storage_plus::{Item};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const VALIDATOR_ADDRESS: Item<String> = Item::new("validator_address");
pub const LOCK_IDS: Item<Vec<u64>> = Item::new("lock_ids");