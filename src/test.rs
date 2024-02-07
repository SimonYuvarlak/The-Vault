#[cfg(test)]
use crate::contract::{execute, instantiate_contract, query};
use crate::msg::{DepositAddressesResponse, ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
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
        sender.clone(),
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
            addresses: vec!["sender".to_string()]
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
        sender.clone(),
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

    assert_eq!(resp, DepositAddressesResponse { addresses: vec![] },);

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
        .query_wasm_smart(contract_addr, &QueryMsg::GetState {})
        .unwrap();

    assert_eq!(resp.total_amount, Uint128::zero(),);

    // Execute - Add Allowance
    // app.execute_contract(
    //     owner.clone(),
    //     contract_addr.clone(),
    //     &ExecuteMsg::AddAllowance {
    //         spender: "sender".to_string(),
    //         amount: Uint128::from(5u128),
    //     },
    //     &[],
    // )
}
