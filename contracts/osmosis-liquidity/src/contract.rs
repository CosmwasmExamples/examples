use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, coins,
    Reply, Response, StdResult, SubMsg, SubMsgResult, SubMsgResponse, Uint128, BankMsg};
use cw2::set_contract_version;

use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgJoinSwapExternAmountIn, MsgExitSwapShareAmountIn, MsgJoinSwapExternAmountInResponse, MsgExitSwapShareAmountInResponse
};
use osmosis_std::types::cosmos::base::v1beta1::Coin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::state::{AddLiquidityState, ADD_LIQUIDITY_STATE, REMOVE_LIQUIDITY_STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-liquidity";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const ADD_LIQUIDITY_REPLY_ID: u64 = 1;
const REMOVE_LIQUIDITY_REPLY_ID: u64 = 2;

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
    Ok(Response::default())
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
        ExecuteMsg::AddLiquidity {
            pool_id, min_shares
        } => execute::add_liquidity(deps, env, info, pool_id, min_shares),
        ExecuteMsg::RemoveLiquidity {
            pool_id, denom_out, min_tokens_out
        } => execute::remove_liquidity(deps, env, info, pool_id, denom_out, min_tokens_out),
    }
}

pub mod execute {
    use crate::state::{ADD_LIQUIDITY_STATE, RemoveLiquidityState};

    use super::*;

    pub fn add_liquidity(
        deps: DepsMut, env: Env, info: MessageInfo, pool_id: u64, min_shares: String
    ) -> Result<Response, ContractError> {
        if info.funds.len() != 1 {
            return Err(ContractError::InvalidFundsRequest {})
        }
        let fund = info.funds[0].clone();
        let join_pool_msg = MsgJoinSwapExternAmountIn {
            sender: env.contract.address.to_string(),
            pool_id,
            token_in: Some(Coin { denom: fund.denom, amount: fund.amount.to_string() }),
            share_out_min_amount: min_shares.clone(),
        };
        ADD_LIQUIDITY_STATE.save(deps.storage, &AddLiquidityState {
            sender: info.sender, pool_id, min_shares })?;
        Ok(Response::new()
            .add_attribute("action", "add_liquidity")
            .add_submessage(SubMsg::reply_on_success(join_pool_msg, ADD_LIQUIDITY_REPLY_ID)),
        )
    }

    pub fn remove_liquidity(
        deps: DepsMut, env: Env, info: MessageInfo, pool_id: u64, denom_out: String, min_tokens_out: String,
    ) -> Result<Response, ContractError> {
        if info.funds.len() != 1 {
            return Err(ContractError::InvalidFundsRequest {})
        }
        let fund = info.funds[0].clone();
        let exit_pool_msg = MsgExitSwapShareAmountIn {
            sender: env.contract.address.to_string(),
            pool_id,
            token_out_denom: denom_out.clone(),
            share_in_amount: fund.amount.to_string(),
            token_out_min_amount: min_tokens_out.clone(),
        };
        REMOVE_LIQUIDITY_STATE.save(deps.storage, &RemoveLiquidityState {
            sender: info.sender,
            pool_id,
            denom_out,
            min_tokens_out,
        })?;
        Ok(Response::new()
            .add_attribute("action", "remove_liquidity")
            .add_submessage(SubMsg::reply_on_success(exit_pool_msg, REMOVE_LIQUIDITY_REPLY_ID))
        )
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        ADD_LIQUIDITY_REPLY_ID => handle_add_liquidity_reply(deps, msg),
        REMOVE_LIQUIDITY_REPLY_ID => handle_remove_liquidity_reply(deps, msg),
        _id => Err(ContractError::CustomError { val: format!("Unknown reply id {}", msg.id) }),
    }
}

fn handle_add_liquidity_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), ..}) = msg.result {
        let state = ADD_LIQUIDITY_STATE.load(deps.storage)?;
        let res: MsgJoinSwapExternAmountInResponse = b.try_into().map_err(ContractError::Std)?;
        let share_out_amount_number = Uint128::from_str(&res.share_out_amount)?;
        let min_shares_number = Uint128::from_str(&state.min_shares)?;
        if share_out_amount_number < min_shares_number {
            return Err(ContractError::CustomError {
                val: format!("share_out < min_share {} {}", share_out_amount_number, min_shares_number)
            })
        }
        let denom_out = format!("gamm/pool/{}", state.pool_id);
        let transfer_msg = BankMsg::Send {
            to_address: state.sender.to_string(),
            amount: coins(share_out_amount_number.u128(), denom_out),
        };
        return Ok(Response::new()
            .add_attribute("share_out_amount", share_out_amount_number)
            .add_message(transfer_msg)
        ) 
    }
    Err(ContractError::FailRemoveLiquidity {
        reason: msg.result.unwrap_err(),
    })
}

fn handle_remove_liquidity_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), ..}) = msg.result {
        let state = REMOVE_LIQUIDITY_STATE.load(deps.storage)?;
        let res: MsgExitSwapShareAmountInResponse = b.try_into().map_err(ContractError::Std)?;
        let token_out_amount_number = Uint128::from_str(&res.token_out_amount)?;
        let min_tokens_out_number = Uint128::from_str(&state.min_tokens_out)?;
        if token_out_amount_number < min_tokens_out_number {
            return Err(ContractError::CustomError {
                val: format!("token_out < min_token_out {} {}", token_out_amount_number, min_tokens_out_number)
            })
        }
        let transfer_msg = BankMsg::Send {
            to_address: state.sender.to_string(),
            amount: coins(token_out_amount_number.u128(), state.denom_out),
        };
        return Ok(Response::new()
            .add_attribute("share_out_amount", token_out_amount_number)
            .add_message(transfer_msg)
        )
    }
    Err(ContractError::FailRemoveLiquidity {
        reason: msg.result.unwrap_err(),
    })
}