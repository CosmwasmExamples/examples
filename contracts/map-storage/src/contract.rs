#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::KEY_VALUE_MAPPING;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:map-storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Set { key, value } => execute::set(deps, key, value),
    }
}

pub mod execute {
    use super::*;
    pub fn set(
        deps: DepsMut, key: String, value: String,
    ) -> Result<Response, ContractError> {
        KEY_VALUE_MAPPING.save(deps.storage, key, &value)?;
        Ok(Response::new())
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMap {  } => to_binary(&query::get_map(deps)?),
        QueryMsg::AllKeys {  } => to_binary(&query::get_all_keys(deps)?),
    }
}

pub mod query {
    use super::*;
    pub fn get_map(deps: Deps) -> StdResult<Vec<(String,String)>> {
        let keys: StdResult<Vec<_>> = KEY_VALUE_MAPPING
            .range(deps.storage, None, None, cosmwasm_std::Order::Descending)
            .collect();
        keys
    }
    pub fn get_all_keys(deps: Deps) -> StdResult<Vec<String>> {
        let keys: StdResult<Vec<_>> = KEY_VALUE_MAPPING
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| {
                let (key, _value) = item?;
                Ok(key)
            })
            .collect();
        keys
    }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    todo!()
}
