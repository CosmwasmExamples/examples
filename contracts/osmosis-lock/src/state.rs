use cosmwasm_std::Addr;
use cw_storage_plus::{Item};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const UNLOCK_ID_STATE: Item<u64> = Item::new("unlock_state");
pub const LOCK_IDS: Item<Vec<u64>> = Item::new("lock_ids_by_users");