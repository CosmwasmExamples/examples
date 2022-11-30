#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use self::query::get_balance;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:transfer";
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
        ExecuteMsg::Deposit {  } => execute::deposit(),
        ExecuteMsg::Withdraw { 
            amount, denom
        } => execute::withdraw(info, amount, denom),
        ExecuteMsg::WithdrawAll {  } => execute::withdraw_all(deps, env, info)
    }
}

pub mod execute {
    use std::str::FromStr;

    use cosmwasm_std::{CosmosMsg, BankMsg, Uint128, coins};

    use super::*;

    pub fn deposit() -> Result<Response, ContractError> {
        Ok(Response::new())
    }

    pub fn withdraw(info: MessageInfo, amount: String, denom: String) -> Result<Response, ContractError> {
        let amount = Uint128::from_str(&amount)?;
        let send_msg: CosmosMsg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: coins(amount.u128(), denom),
        }.into();
        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_message(send_msg)
        )
    }

    pub fn withdraw_all(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let contract_address = env.contract.address.to_string();
        let balances = deps.querier.query_all_balances(contract_address)?;
        let transfer_msg: CosmosMsg = BankMsg::Send {
            to_address: info.sender.to_string(), amount: balances
        }.into();
        Ok(Response::new().add_message(transfer_msg))
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {} => to_binary(&get_balance(deps, env)?)
    }
}

pub mod query {
    use cosmwasm_std::Coin;

    use super::*;

    pub fn get_balance(deps: Deps, env: Env) -> StdResult<Vec<Coin>> {
        let balances = deps.querier.query_all_balances(env.contract.address.to_string())?;
        Ok(balances)
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
