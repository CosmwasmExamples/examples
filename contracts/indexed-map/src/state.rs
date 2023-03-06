use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{MultiIndex, IndexList, Index, IndexedMap};

#[cw_serde]
pub struct UserRecord {
  pub id: u64,
  pub owner: Addr,
  pub amount: Uint128,
}

pub struct UserIndexes<'a> {
  pub owner: MultiIndex<'a, Addr, UserRecord, u64>
}

impl IndexList<UserRecord> for UserIndexes<'_> {
  fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn cw_storage_plus::Index<UserRecord>> + '_> {
      let v: Vec<&dyn Index<UserRecord>> = vec![&self.owner];

      Box::new(v.into_iter())
  }
}
pub fn unlocks<'a>() -> IndexedMap<'a, u64, UserRecord, UserIndexes<'a>> {
  let indexes = UserIndexes {
    owner: MultiIndex::new(
      |t| t.owner.clone(),
      "USER_RECORD",
      "USER_RECORD_OWNER",
    ),
  };

  IndexedMap::new("USER_RECORD", indexes)
}