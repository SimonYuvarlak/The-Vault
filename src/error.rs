use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call this function")]
    NotOwner { owner: String },

    #[error("This address cannot deposit")]
    UnauthorizedDepositAddress { address: String },

    #[error("This address has no allowance")]
    NoAllowance { address: String },

    #[error("This address has 0 allowance")]
    ZeroAllowance { address: String },

    #[error("This address has not enough allowance")]
    NotEnoughAllowance { address: String },

    #[error("Vault does not have enough funds")]
    NotEnoughFunds { total_amount: u128 },

    #[error("Deposit address already exists")]
    DepositAddressExists { id: u64 },

    #[error("Address has already been added to the allowance list")]
    AllowanceExists { address: String },
}
