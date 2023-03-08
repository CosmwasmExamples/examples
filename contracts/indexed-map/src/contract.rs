#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    Addr, Uint128
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{users, UserRecord};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:indexed-map";
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

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    todo!()
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Set { id, amount } => execute::set(deps, info, id, amount),    
    }
}

mod execute {
    use super::*;
    pub fn set(deps: DepsMut, info: MessageInfo, id: u64, amount: Uint128) -> Result<Response, ContractError> {
        users().save(deps.storage, id, &UserRecord {
            id, amount, owner: info.sender,
        })?;
        Ok(Response::new())
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Get { id } => to_binary(&query::get(deps, id)?),
        QueryMsg::GetByOwner { owner } => to_binary(&query::get_by_owner(deps, owner)?),
    }   
}

mod query {
    use super::*;

    pub fn get(deps: Deps, id: u64) -> StdResult<UserRecord> {
        let res = users().load(deps.storage, id)?;
        Ok(res)
    }

    pub fn get_by_owner(deps: Deps, owner: Addr) -> StdResult<Vec<UserRecord>> {
        let res = users()
            .idx.owner
            .prefix(owner)
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|data| {
                let (_key, value) = data.unwrap();
                value
            })
            .collect();
        Ok(res)
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}

