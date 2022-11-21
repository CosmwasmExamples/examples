#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, coins,
    Reply, Response, StdResult, SubMsg, SubMsgResult, SubMsgResponse, BankMsg, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::state::SWAP_REPLY_STATES;

use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgSwapExactAmountIn, SwapAmountInRoute, MsgSwapExactAmountInResponse
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmosis-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SWAP_REPLY_ID: u64 = 1;

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

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Swap {
            pool_id, amount_out_min, denom_out
        } => execute::swap(deps, env, info, pool_id, amount_out_min, denom_out),
    }
}

pub mod execute {
    use cosmwasm_std::CosmosMsg;

    use crate::state::{SWAP_REPLY_STATES, SwapMsgReplyState};

    use super::*;

    pub fn swap(deps: DepsMut, env: Env, info: MessageInfo, pool_id: u64, amount_out_min: String, denom_out: String) -> Result<Response, ContractError> {
        if info.funds.is_empty() {
            return Err(ContractError::InvalidFund {});
        }
        let fund = info.funds[0].clone();
        let route = SwapAmountInRoute {
            pool_id,
            token_out_denom: denom_out,
        };
        let swap_msg: CosmosMsg = MsgSwapExactAmountIn {
            sender: env.contract.address.to_string(),
            routes: vec![route],
            token_in: Some(Coin { denom: fund.denom, amount: fund.amount.to_string() }),
            token_out_min_amount: amount_out_min
        }.into();

        // SWAP_REPLY_STATES.save(deps.storage, &SwapMsgReplyState {
        //     sender: info.sender,
        //     denom_out,
        // })?;
        Ok(Response::new()
            .add_attribute("method", "swap")
            .add_message(swap_msg))
        // let submessage = SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID);
        // Ok(Response::new()
        //     .add_attribute("method", "swap")
        //     .add_submessage(submessage))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // perform state update or anything neccessary for the migration
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError>  {
    match msg.id {
        SWAP_REPLY_ID => handle_swap_reply(deps, msg),
        _id => Err(ContractError::InvalidReplyID {  }),
    }
}

fn handle_swap_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    return Ok(Response::new());
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        return Ok(Response::new())
        // let swap_state = SWAP_REPLY_STATES.load(deps.storage)?;
        // let swap_result: MsgSwapExactAmountInResponse = b.try_into().map_err(ContractError::Std)?;
        // let amount_out = Uint128::from_str(&swap_result.token_out_amount)?;
        // let transfer_msg = BankMsg::Send {
        //     to_address: swap_state.sender.to_string(), amount: coins(amount_out.u128(), swap_state.denom_out),
        // };
        // return Ok(Response::new()
        //     .add_attribute("token_out_amount", amount_out)
        //     .add_message(transfer_msg)
        // )
    }
    Err(ContractError::FailedSwap {
        reason: msg.result.unwrap_err(),
    })
}