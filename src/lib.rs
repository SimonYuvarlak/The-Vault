use contract::instantiate_contract;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;
use msg::{ExecuteMsg, InstantiateMsg};

mod contract;
mod error;
pub mod msg;
mod state;
mod test;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    instantiate_contract(deps, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => contract::execute::deposit_token(deps, info),
        ExecuteMsg::AddDepositAddress { address } => {
            contract::execute::add_deposit_address(deps, info, address)
        }
        ExecuteMsg::RemoveDepositAddress { address } => {
            contract::execute::remove_deposit_address(deps, info, address)
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

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;

    match msg {
        GetState {} => to_json_binary(&contract::query::get_state(deps)?),
        GetAllowance { spender } => to_json_binary(&contract::query::get_allowance(deps, spender)?),
        GetAllowances {} => to_json_binary(&contract::query::get_allowances(deps)?),
        CanDeposit { address } => to_json_binary(&contract::query::can_deposit(deps, address)?),
        GetDepositAddresses {} => to_json_binary(&contract::query::get_deposit_addresses(deps)?),
    }
}
