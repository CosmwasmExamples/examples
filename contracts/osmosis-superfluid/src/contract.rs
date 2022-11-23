#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsgResponse, SubMsgResult};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::superfluid::MsgLockAndSuperfluidDelegateResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{OWNER, VALIDATOR_ADDRESS, LOCK_IDS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-superfluid";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOCK_AND_DELEGATE_REPLY_ID: u64 = 1;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let owner_address = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner_address)?;
    VALIDATOR_ADDRESS.save(deps.storage, &msg.validator_address)?;
    LOCK_IDS.save(deps.storage, &vec![])?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner_address.to_string()))
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
        ExecuteMsg::LockAndDelegate {  } => execute::lock_and_delegate(deps, env, info),
        ExecuteMsg::UndelegateAndUnbond {
            lock_id
        } => execute::undelegate_and_unbond(deps, env, info, lock_id),
        ExecuteMsg::Withdraw {
            amount, denom
        } => execute::withdraw(deps, info, amount, denom),
    }
}

pub mod execute {
    use std::str::FromStr;

    use super::*;
    use cosmwasm_std::{CosmosMsg, Uint128, coins, BankMsg, SubMsg};
    use osmosis_std::types::{osmosis::superfluid::{
        MsgLockAndSuperfluidDelegate, MsgSuperfluidUndelegate, MsgSuperfluidUnbondLock
    }, cosmos::base::v1beta1::Coin};

    pub fn lock_and_delegate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  });
        }
        let validator_address = VALIDATOR_ADDRESS.load(deps.storage)?;
        let fund = info.funds[0].clone();
        let coins: Vec<Coin> = vec![Coin { denom: fund.denom, amount: fund.amount.to_string() }];
        let lock_and_delegate_msg: CosmosMsg = MsgLockAndSuperfluidDelegate {
            sender: env.contract.address.to_string(),
            coins,
            val_addr: validator_address,
        }.into();
        let submessage = SubMsg::reply_on_success(lock_and_delegate_msg, LOCK_AND_DELEGATE_REPLY_ID);
        Ok(Response::new()
            .add_attribute("action", "lock_and_delegate")
            .add_submessage(submessage)
        )
    }

    pub fn undelegate_and_unbond(deps: DepsMut, env: Env, info: MessageInfo, lock_id: u64) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {  });
        }
        let undelegate_msg: CosmosMsg = MsgSuperfluidUndelegate {
            sender: env.contract.address.to_string(),
            lock_id,
        }.into();
        let unbond_msg: CosmosMsg = MsgSuperfluidUnbondLock {
            sender: env.contract.address.to_string(),
            lock_id,
        }.into();
        LOCK_IDS.update(deps.storage, |mut locks| -> Result<_, ContractError> {
            let index = locks.iter().position(|x| *x == lock_id);
            if let Some(i) = index {
                locks.swap_remove(i);
            }
            Ok(locks)
        })?;
        Ok(Response::new()
            .add_attribute("action", "undelegate_and_unbond")
            .add_message(undelegate_msg)
            .add_message(unbond_msg)
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLocks {  } => to_binary(&query::get_locks(deps)?),
    }
}

pub mod query {
    use super::*;
    use crate::{msg::GetLockResponse};

    pub fn get_locks(deps: Deps) -> StdResult<GetLockResponse> {
        let locks = LOCK_IDS.load(deps.storage)?;
        Ok(GetLockResponse {
            locks,
        })
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        LOCK_AND_DELEGATE_REPLY_ID => handle_lock_and_delegate_reply(deps, msg),
        _id => Err(ContractError::CustomError { val: format!("Unknow reply id {}", msg.id) })
    }
}

fn handle_lock_and_delegate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), ..}) = msg.result {
        let res: MsgLockAndSuperfluidDelegateResponse = b.try_into().map_err(ContractError::Std)?;
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
    Err(ContractError::FailToLockAndDelegate {
        reason: msg.result.unwrap_err(),
    })
}