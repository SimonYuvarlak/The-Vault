use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use error::ContractError;
use msg::ExecuteMsg;

mod contract;
mod error;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { coin } => contract::execute::deposit_token(deps, info, coin),
        ExecuteMsg::AddDepositAddress { address } => {
            contract::execute::add_deposit_address(deps, info, address)
        }
        ExecuteMsg::Withdraw {} => contract::execute::withdraw(deps, env, info),
        ExecuteMsg::AddAllowance { spender, amount } => {
            contract::execute::add_allowance(deps, info, spender, amount)
        }
        ExecuteMsg::AddAllowanceList { spenders, amounts } => {
            contract::execute::add_allowance_list(deps, info, spenders, amounts)
        }
        ExecuteMsg::RemoveAllowance { spender } => {
            contract::execute::remove_allowance(deps, info, spender)
        }
        ExecuteMsg::UpdateAllowance { spender, amount } => {
            contract::execute::update_allowance(deps, info, spender, amount)
        }
        ExecuteMsg::RetrieveAllowance {} => contract::execute::retrieve_allowance(deps, info),
        ExecuteMsg::UpdateName { name } => contract::execute::update_name(deps, info, name),
        ExecuteMsg::UpdateOwner { owner } => contract::execute::update_owner(deps, info, owner),
    }
}

// #[entry_point]
// pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
//     use contract::query;
//     use msg::QueryMsg::*;

//     match msg {
//         Value {} => to_json_binary(&query::value()),
//     }
// }
