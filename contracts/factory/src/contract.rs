#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw0::parse_reply_instantiate_data;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, WasmMsg, CosmosMsg, StdError,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, YourNameResponse};

use crate::state::{Name, NAMES, CHILD_CONTRACT_CODE_ID, REPLY_STORAGE, InstantiateReplyState};

use myname;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_REPLY_ID:u64 = 1;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CHILD_CONTRACT_CODE_ID.save(deps.storage, &msg.contract_code_id)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
    }
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
        ExecuteMsg::AddName { name } => execute::add_name(deps, info, name),
    }
}

pub mod execute {
    use cosmwasm_std::SubMsg;

    use super::*;

    pub fn add_name(deps: DepsMut, info: MessageInfo, name: String) -> Result<Response, ContractError> {
        let code_id = CHILD_CONTRACT_CODE_ID.load(deps.storage)?;
        let instantiate_message: WasmMsg = WasmMsg::Instantiate {
            admin: None,
            code_id,
            msg: to_binary(&myname::msg::InstantiateMsg {
                name: name.clone(),
            })?,
            funds: vec![],
            label: name.clone(),
        };
        REPLY_STORAGE.save(deps.storage, INSTANTIATE_REPLY_ID, &InstantiateReplyState {
            name,
            owner: info.sender,
        })?;
        let submessage = SubMsg::reply_on_success(CosmosMsg::from(instantiate_message), INSTANTIATE_REPLY_ID);
        Ok(Response::new()
            .add_attribute("action", "add_name")
            .add_submessage(submessage)
        )
    }
}
/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::YourName { owner } => to_binary(&query::name(deps, owner)?),
    }
}

pub mod query {
    use cosmwasm_std::Addr;

    use super::*;

    pub fn name(deps: Deps, owner: String) -> StdResult<YourNameResponse> {
        let owner: Addr = deps.api.addr_validate(&owner)?;
        let data = NAMES.load(deps.storage, &owner)?;
        Ok(YourNameResponse { name: data.name, contract: data.contract })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let state = REPLY_STORAGE.load(deps.storage, msg.id)?;
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg, state),
        id => Err(StdError::generic_err(format!("Unknow reply id: {}", id))),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply, state: InstantiateReplyState) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg).map_err(|err| StdError::generic_err(err.to_string()))?;
    let child_contract = deps.api.addr_validate(&res.contract_address)?;
    NAMES.save(deps.storage, &state.owner, &Name {
        name: state.name,
        contract: child_contract.to_string(),
    })?;
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, SubMsg, ReplyOn};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        
        let msg = InstantiateMsg { contract_code_id: 1 };
        let info = mock_info("creator", &Vec::new());

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(res.attributes[0].value, "instantiate");
        assert_eq!(res.attributes[1].value, "creator");
    }

    #[test]
    fn execute_add_name() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { contract_code_id: 1 };
        let info = mock_info("creator", &vec![]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("anyone", &vec![]);
        let msg = ExecuteMsg::AddName { name: "Test".to_string() };
        let execute_res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let state_res = REPLY_STORAGE.load(&deps.storage, INSTANTIATE_REPLY_ID).unwrap();
        assert_eq!(state_res, InstantiateReplyState {
            name: "Test".to_string(),
            owner: Addr::unchecked("anyone"),
        });
        let instantiate_msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
            admin: None,
            code_id: 1,
            msg: to_binary(&myname::msg::InstantiateMsg {
                name: "Test".to_string()
            })
            .unwrap(),
            funds: vec![],
            label: "Test".to_string()
        });
        assert_eq!(execute_res.messages, vec![SubMsg {
            gas_limit: None,
            id: INSTANTIATE_REPLY_ID,
            reply_on: ReplyOn::Success,
            msg: instantiate_msg.into(),
        }]);
    }
}