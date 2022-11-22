#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsgResult, SubMsgResponse, to_binary,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::lockup::{MsgLockTokensResponse, MsgBeginUnlockingResponse};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::state::{OWNER, LOCK_IDS, UNLOCK_ID_STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-lock";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOCK_REPLY_ID: u64 = 1;
const UNLOCK_REPLY_ID: u64 = 2;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OWNER.save(deps.storage, &info.sender)?;
    LOCK_IDS.save(deps.storage, &vec![])?;
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Lock {
            duration,
        } => execute::lock(deps, env, info, duration),
        ExecuteMsg::Unlock {
            lock_id,
        } => execute::unlock(deps, env, info, lock_id),
        ExecuteMsg::Withdraw {
            amount, denom
        } => execute::withdraw(deps, info, amount, denom),
    }
}

pub mod execute {
    use super::*;
    use std::str::FromStr;

    use cosmwasm_std::{MessageInfo, CosmosMsg, Uint128, coins, SubMsg, BankMsg};
    use osmosis_std::{shim::Duration};
    use osmosis_std::types::cosmos::base::v1beta1::Coin;
    use osmosis_std::types::osmosis::lockup::{
        MsgLockTokens, MsgBeginUnlocking,
    };

    use crate::ContractError;
    use crate::state::{OWNER, UNLOCK_ID_STATE};

    use super::LOCK_REPLY_ID;

    pub fn lock(deps: DepsMut, env: Env, info: MessageInfo, duration: u64) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  });
        }
        let fund = info.funds[0].clone();
        let coins: Vec<Coin> = vec![Coin { denom: fund.denom, amount: fund.amount.to_string() }];
        let lock_msg: CosmosMsg = MsgLockTokens {
            owner: env.contract.address.to_string(),
            duration: Some(Duration {
                seconds: duration as i64,
                nanos: 0,
            }),
            coins,
        }.into();
        let submessage = SubMsg::reply_on_success(lock_msg, LOCK_REPLY_ID);
        Ok(Response::new()
            .add_attribute("action", "lock")
            .add_submessage(submessage)
        )
    }

    pub fn unlock(deps: DepsMut, env: Env, info: MessageInfo, lock_id: u64) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  });
        }
        let unlock_msg: CosmosMsg = MsgBeginUnlocking {
            owner: env.contract.address.to_string(),
            id: lock_id,
            coins: vec![],
        }.into();
        UNLOCK_ID_STATE.save(deps.storage, &lock_id)?;
        let submessage = SubMsg::reply_on_success(unlock_msg, UNLOCK_REPLY_ID);
        Ok(Response::new()
            .add_attribute("action", "unlock")
            .add_submessage(submessage)
        )
    }

    pub fn withdraw(deps: DepsMut, info: MessageInfo, amount: String, denom: String) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  });
        }
        let amount = Uint128::from_str(&amount)?;
        let send_msg: CosmosMsg = BankMsg::Send {
            to_address: info.sender.to_string(), amount: coins(amount.u128(), denom)
        }.into();
        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_message(send_msg)
        )
    }

}
/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLocks {  } => to_binary(&query::get_locks(deps)?),
    }
}

pub mod query {
    use super::*;
    use crate::msg::GetLockResponse;

    pub fn get_locks(deps: Deps) -> StdResult<GetLockResponse> {
        let locks = LOCK_IDS.load(deps.storage)?;
        Ok(GetLockResponse {
            locks,
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        LOCK_REPLY_ID => handle_lock_reply(deps, msg),
        UNLOCK_REPLY_ID => handle_unlock_reply(deps, msg),
        _id => Err(ContractError::CustomError { val: format!("Unknow reply id {}", msg.id) }),
    }
}

fn handle_lock_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), ..}) = msg.result {
        let res: MsgLockTokensResponse = b.try_into().map_err(ContractError::Std)?;
        LOCK_IDS.update(deps.storage, |mut locks| -> Result<_, ContractError> {
            if !locks.contains(&res.id) {
                locks.push(res.id);
            }
            Ok(locks)
        })?;
        return Ok(Response::new()
            .add_attribute("lock_id", res.id.to_string())
        ) 
    }
    Err(ContractError::FailToLock {
        reason: msg.result.unwrap_err(),
    })
}

fn remove_unlock_id(deps: DepsMut) -> Result<u64, ContractError> {
    let unlock_id = UNLOCK_ID_STATE.load(deps.storage)?;
    LOCK_IDS.update(deps.storage, |mut locks| -> Result<_, ContractError> {
        let index = locks.iter().position(|x| *x == unlock_id).unwrap();
        locks.swap_remove(index);
        Ok(locks)
    })?;
    Ok(unlock_id)
}

fn handle_unlock_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data, ..}) = msg.result {
        if let Some(b) = data {
            let res: MsgBeginUnlockingResponse = b.try_into().map_err(ContractError::Std)?;
            if res.success {
                let unlock_id = remove_unlock_id(deps)?;
                return Ok(Response::new().add_attribute("unlock_id", unlock_id.to_string()));
            } else {
                return Err(ContractError::FailToUnlock { reason: "Fail".to_string() })
            }
        } else { // no binary in response, is it a bug?
            let unlock_id = remove_unlock_id(deps)?;
            return Ok(Response::new().add_attribute("unlock_id", unlock_id.to_string()));
        }
    }
    Err(ContractError::FailToUnlock {
        reason: msg.result.unwrap_err(),
    })
}
