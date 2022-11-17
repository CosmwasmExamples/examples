use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
  pub name: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
  #[returns(GetContractResponse)]
  GetContract {},
}

#[cw_serde]
pub struct GetContractResponse {
  pub name: String,
  pub owner: String,
}