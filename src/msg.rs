use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub expected_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    AddDepositAddress {
        address: String,
    },
    RemoveDepositAddress {
        address: String,
    },
    Withdraw {},
    AddAllowance {
        spender: String,
        amount: Uint128,
    },
    AddAllowanceList {
        spenders: Vec<String>,
        amounts: Vec<Uint128>,
    },
    RemoveAllowance {
        spender: String,
    },
    UpdateAllowance {
        spender: String,
        amount: Uint128,
    },
    RetrieveAllowance {},
    UpdateName {
        name: String,
    },
    UpdateOwner {
        owner: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StateResponse)]
    GetState {},
    #[returns(AllowanceResponse)]
    GetAllowance { spender: String },
    #[returns(AllowancesResponse)]
    GetAllowances {},
    #[returns(CanDepositResponse)]
    CanDeposit { address: String },
    #[returns(DepositAddressesResponse)]
    GetDepositAddresses {},
}

#[cw_serde]
pub struct StateResponse {
    pub owner: String,
    pub name: String,
    pub total_amount: Uint128,
    pub expected_denom: String,
}

#[cw_serde]
pub struct AllowanceResponse {
    pub spender: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct AllowancesResponse {
    pub spenders: Vec<String>,
    pub amounts: Vec<Uint128>,
}

#[cw_serde]
pub struct CanDepositResponse {
    pub can_deposit: bool,
}

#[cw_serde]
pub struct DepositAddressesResponse {
    pub addresses: Vec<String>,
}
