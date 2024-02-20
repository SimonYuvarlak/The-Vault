#[cfg(test)]
use crate::contract::{execute, instantiate_contract, query};
use crate::msg::{
    AllowanceResponse, AllowancesResponse, DepositAddressesResponse, ExecuteMsg, InstantiateMsg,
    QueryMsg, StateResponse,
};
use crate::{execute, instantiate, query};
use cosmwasm_std::{
    coins,
    testing::{mock_env, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    Addr, Empty, Uint128,
};
use cw_multi_test::{App, BankKeeper, Contract, ContractWrapper, Executor};

fn vault_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn multitest_vault_contract() {
    // Create addresses
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    // Create mock app
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(100, "atom"))
            .unwrap();
    });

    // Create contract id
    let contract_id = app.store_code(vault_contract());

    // Instantiate - Get contract address
    let contract_addr = app
        .instantiate_contract(
            contract_id,
            owner.clone(),
            &InstantiateMsg {
                name: "Vault X".to_string(),
                expected_denom: "atom".to_string(),
            },
            &[],
            "Vault contract",
            None,
        )
        .unwrap();

    // Execute - Add deposit address
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::AddDepositAddress {
            address: "sender".to_string(),
        },
        &[],
    )
    .unwrap();

    // Query - Get deposit addresses
    let resp: DepositAddressesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetDepositAddresses {})
        .unwrap();

    assert_eq!(
        resp,
        DepositAddressesResponse {
            addresses: vec!["owner".to_string(), "sender".to_string()]
        },
    );

    // Execute - Deposit
    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &ExecuteMsg::Deposit {},
        &coins(10, "atom"),
    )
    .unwrap();

    // Query - Get token balance
    let resp: StateResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetState {})
        .unwrap();

    assert_eq!(resp.total_amount, Uint128::from(10u128),);

    // Execute - Remove deposit address
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::RemoveDepositAddress {
            address: "sender".to_string(),
        },
        &[],
    )
    .unwrap();

    // Query - Get deposit addresses
    let resp: DepositAddressesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetDepositAddresses {})
        .unwrap();

    assert_eq!(
        resp,
        DepositAddressesResponse {
            addresses: vec!["owner".to_string()]
        },
    );

    // Execute - Withdraw
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::Withdraw {},
        &[],
    )
    .unwrap();

    // Query - Get token balance
    let resp: StateResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetState {})
        .unwrap();

    assert_eq!(resp.total_amount, Uint128::zero(),);

    // Execute - Add Allowance
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::AddAllowance {
            spender: "sender".to_string(),
            amount: Uint128::from(5u128),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowance
    let resp: AllowanceResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::GetAllowance {
                spender: "sender".to_string(),
            },
        )
        .unwrap();

    assert_eq!(resp.spender, "sender".to_string(),);
    assert_eq!(resp.amount, Uint128::from(5u128),);

    // Execute - Add Allowance
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::AddAllowance {
            spender: "receiver".to_string(),
            amount: Uint128::from(10u128),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowance List
    let resp: AllowancesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetAllowances {})
        .unwrap();

    assert_eq!(
        resp.spenders,
        vec!["receiver".to_string(), "sender".to_string()],
    );
    assert_eq!(
        resp.amounts,
        vec![Uint128::from(10u128), Uint128::from(5u128)],
    );

    // Remove Allowance
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::RemoveAllowance {
            spender: "receiver".to_string(),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowance List
    let resp: AllowancesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetAllowances {})
        .unwrap();

    assert_eq!(resp.spenders, vec!["sender".to_string()],);
    assert_eq!(resp.amounts, vec![Uint128::from(5u128)],);

    // Update Allowance
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::UpdateAllowance {
            spender: "sender".to_string(),
            amount: Uint128::from(15u128),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowances
    let resp: AllowancesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetAllowances {})
        .unwrap();

    assert_eq!(resp.spenders, vec!["sender".to_string()],);
    assert_eq!(resp.amounts, vec![Uint128::from(15u128)],);

    // Remove Allowance - The list is empty after this call
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::RemoveAllowance {
            spender: "sender".to_string(),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowances
    let resp: AllowancesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetAllowances {})
        .unwrap();

    assert_eq!(resp.spenders, Vec::<String>::new(),);
    assert_eq!(resp.amounts, Vec::<Uint128>::new(),);

    // Create list of allowances. The spenders and amounts will be separate vectors
    let mut allowance_spenders = vec!["joel".to_string(), "ellie".to_string()];
    let mut allowance_amounts = vec![Uint128::from(10u128), Uint128::from(20u128)];

    // Add Allowance List
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::AddAllowanceList {
            spenders: allowance_spenders.clone(),
            amounts: allowance_amounts.clone(),
        },
        &[],
    )
    .unwrap();

    // Query - Get Allowance List
    let resp: AllowancesResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetAllowances {})
        .unwrap();

    allowance_spenders.reverse();
    allowance_amounts.reverse();

    assert_eq!(resp.spenders, allowance_spenders,);
    assert_eq!(resp.amounts, allowance_amounts,);

    // Execute - Deposit
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::Deposit {},
        &coins(10, "atom"),
    )
    .unwrap();

    // Execute - Add Allowance
    app.execute_contract(
        owner.clone(),
        contract_addr.clone(),
        &ExecuteMsg::AddAllowance {
            spender: "ellie".to_string(),
            amount: Uint128::from(10u128),
        },
        &[],
    )
    .unwrap();

    // Retrieve Allowance
    app.execute_contract(
        Addr::unchecked("ellie"),
        contract_addr.clone(),
        &ExecuteMsg::RetrieveAllowance {},
        &[],
    )
    .unwrap();

    // Query - Get token balance
    let resp: StateResponse = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetState {})
        .unwrap();

    assert_eq!(resp.total_amount, Uint128::zero(),);

    // Update Name

    // Update Owner
}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {
//     #[returns(StateResponse)]
//     GetState {},
//     #[returns(AllowanceResponse)]
//     GetAllowance { spender: String },
//     #[returns(AllowancesResponse)]
//     GetAllowances {},
//     #[returns(CanDepositResponse)]
//     CanDeposit { address: String },
//     #[returns(DepositAddressesResponse)]
//     GetDepositAddresses {},
// }
