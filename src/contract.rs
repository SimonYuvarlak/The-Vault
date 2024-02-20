use crate::msg::InstantiateMsg;
use crate::state::{State, DEPOSIT_ADDRESSES, STATE};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, Uint128};

pub fn instantiate_contract(
    deps: DepsMut,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let initial_state = State {
        owner: info.clone().sender,
        name: msg.name,
        total_amount: Uint128::zero(),
        expected_denom: msg.expected_denom,
    };
    STATE.save(deps.storage, &initial_state)?;
    DEPOSIT_ADDRESSES.save(deps.storage, info.sender, &Uint128::zero())?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub mod execute {
    use crate::error::ContractError;
    use crate::error::ContractError::UnauthorizedDepositAddress;
    use crate::state::{ALLOWANCES, DEPOSIT_ADDRESSES, STATE};
    use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

    pub fn deposit_token(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let mut current_state = STATE.load(deps.storage)?;
        let mut current_coin = Coin {
            denom: "".to_string(),
            amount: Uint128::zero(),
        };
        if info.funds.iter().any(|coin| {
            current_coin = coin.clone();
            coin.denom == current_state.expected_denom
        }) {
            let address = info.sender;
            if current_coin.denom != current_state.expected_denom {
                return Err(ContractError::InvalidDenom {
                    denom: current_coin.denom.to_string(),
                });
            }
            let address_amount = DEPOSIT_ADDRESSES.may_load(deps.storage, address.clone())?;
            match address_amount {
                Some(value) => {
                    DEPOSIT_ADDRESSES.save(
                        deps.storage,
                        address.clone(),
                        &value.checked_add(current_coin.amount).unwrap_or(value),
                    )?;
                    current_state.total_amount = current_state
                        .total_amount
                        .checked_add(current_coin.amount)
                        .unwrap_or(current_state.total_amount);

                    STATE.save(deps.storage, &current_state)?;
                }
                None => {
                    return Err(UnauthorizedDepositAddress {
                        address: address.clone().to_string(),
                    });
                }
            }
            Ok(Response::new()
                .add_attribute("action", "deposit")
                .add_attribute("address", address.to_string())
                .add_attribute("amount", current_coin.amount))
        } else {
            Err(ContractError::InvalidDenom {
                denom: current_state.expected_denom,
            })
        }
    }

    pub fn add_deposit_address(
        deps: DepsMut,
        info: MessageInfo,
        deposit_address: String,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        let address = match deps.api.addr_validate(deposit_address.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(ContractError::NotValidAddress {
                    address: deposit_address,
                })
            }
        };
        DEPOSIT_ADDRESSES.save(deps.storage, address, &Uint128::zero())?;
        Ok(Response::new()
            .add_attribute("action", "add_deposit_address")
            .add_attribute("address", deposit_address))
    }

    pub fn remove_deposit_address(
        deps: DepsMut,
        info: MessageInfo,
        deposit_address: String,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        let address = match deps.api.addr_validate(deposit_address.as_str()) {
            Ok(value) => value,
            Err(_) => {
                return Err(ContractError::NotValidAddress {
                    address: deposit_address,
                })
            }
        };
        DEPOSIT_ADDRESSES.remove(deps.storage, address);
        Ok(Response::new()
            .add_attribute("action", "remove_deposit_address")
            .add_attribute("address", deposit_address))
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let mut current_state = STATE.load(deps.storage)?;

        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }

        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };

        current_state.total_amount = Uint128::zero();
        STATE.save(deps.storage, &current_state)?;

        Ok(Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw"))
    }

    pub fn add_allowance(
        deps: DepsMut,
        info: MessageInfo,
        spender: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        let address = match deps.api.addr_validate(spender.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(ContractError::NotValidAddress { address: spender }),
        };
        ALLOWANCES.save(deps.storage, address, &amount)?;
        Ok(Response::new()
            .add_attribute("action", "add_allowance")
            .add_attribute("spender", spender)
            .add_attribute("amount", amount.to_string()))
    }

    pub fn add_allowance_list(
        deps: DepsMut,
        info: MessageInfo,
        spenders: Vec<String>,
        amounts: Vec<Uint128>,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        if spenders.len() != amounts.len() {
            return Err(ContractError::AllowanceAddressesAmountsNotEqual {});
        }
        spenders
            .iter()
            .enumerate()
            .try_for_each(|(index, spender)| {
                let address = match deps.api.addr_validate(spender.as_str()) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(ContractError::NotValidAddress {
                            address: spender.clone(),
                        })
                    }
                };
                ALLOWANCES
                    .save(deps.storage, address, &amounts[index])
                    .map_err(ContractError::Std)
            })?;
        Ok(Response::new().add_attribute("action", "add_allowance_list"))
    }

    pub fn remove_allowance(
        deps: DepsMut,
        info: MessageInfo,
        spender: String,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        let address = match deps.api.addr_validate(spender.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(ContractError::NotValidAddress { address: spender }),
        };
        ALLOWANCES.remove(deps.storage, address);
        Ok(Response::new()
            .add_attribute("action", "remove_allowance")
            .add_attribute("spender", spender))
    }

    pub fn update_allowance(
        deps: DepsMut,
        info: MessageInfo,
        spender: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        let address = match deps.api.addr_validate(spender.as_str()) {
            Ok(value) => value,
            Err(_) => return Err(ContractError::NotValidAddress { address: spender }),
        };
        ALLOWANCES.save(deps.storage, address, &amount)?;
        Ok(Response::new()
            .add_attribute("action", "update_allowance")
            .add_attribute("spender", spender)
            .add_attribute("amount", amount.to_string()))
    }

    pub fn retrieve_allowance(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let mut current_state = STATE.load(deps.storage)?;
        let allowance = match ALLOWANCES.load(deps.storage, info.clone().sender) {
            Ok(value) => value,
            Err(_) => {
                return Err(ContractError::NoAllowance {
                    address: info.sender.to_string(),
                })
            }
        };

        current_state.total_amount = current_state
            .total_amount
            .checked_sub(allowance)
            .unwrap_or(current_state.total_amount);

        STATE.save(deps.storage, &current_state)?;

        let bank_msg = BankMsg::Send {
            to_address: info.clone().sender.to_string(),
            amount: vec![Coin {
                denom: current_state.expected_denom,
                amount: allowance,
            }],
        };
        Ok(Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "retrieve_allowance")
            .add_attribute("address", info.sender.to_string())
            .add_attribute("amount", allowance))
    }

    pub fn update_name(
        deps: DepsMut,
        info: MessageInfo,
        name: String,
    ) -> Result<Response, ContractError> {
        let mut current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        current_state.name = name;
        STATE.save(deps.storage, &current_state)?;
        Ok(Response::new().add_attribute("action", "update_name"))
    }

    pub fn update_owner(
        deps: DepsMut,
        info: MessageInfo,
        owner: String,
    ) -> Result<Response, ContractError> {
        let mut current_state = STATE.load(deps.storage)?;
        if info.sender != current_state.owner {
            return Err(ContractError::NotOwner {
                owner: current_state.owner.to_string(),
            });
        }
        current_state.owner = deps.api.addr_validate(owner.as_str())?;
        STATE.save(deps.storage, &current_state)?;
        Ok(Response::new()
            .add_attribute("action", "update_owner")
            .add_attribute("owner", owner))
    }
}

pub mod query {
    use crate::{
        msg::{
            AllowanceResponse, AllowancesResponse, CanDepositResponse, DepositAddressesResponse,
            StateResponse,
        },
        state::{ALLOWANCES, DEPOSIT_ADDRESSES, STATE},
    };
    use cosmwasm_std::{Addr, Deps, StdResult, Uint128};

    pub fn get_state(deps: Deps) -> StdResult<StateResponse> {
        let current_state = STATE.load(deps.storage)?;
        Ok(StateResponse {
            owner: current_state.owner.to_string(),
            name: current_state.name,
            total_amount: current_state.total_amount,
            expected_denom: current_state.expected_denom,
        })
    }

    pub fn get_allowance(deps: Deps, spender: String) -> StdResult<AllowanceResponse> {
        let address = deps.api.addr_validate(spender.as_str())?;
        let amount = ALLOWANCES.load(deps.storage, address)?;
        Ok(AllowanceResponse { spender, amount })
    }

    pub fn get_allowances(deps: Deps) -> StdResult<AllowancesResponse> {
        let spenders = ALLOWANCES
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| {
                let (address, _) = item?;
                Ok(address)
            })
            .collect::<StdResult<Vec<Addr>>>()?;
        let amounts = spenders
            .iter()
            .map(|spender| ALLOWANCES.load(deps.storage, spender.clone()))
            .collect::<StdResult<Vec<Uint128>>>()?;
        Ok(AllowancesResponse {
            spenders: spenders.iter().map(|x| x.to_string()).collect(),
            amounts,
        })
    }

    pub fn can_deposit(deps: Deps, address: String) -> StdResult<CanDepositResponse> {
        let address = deps.api.addr_validate(address.as_str())?;
        let address_amount = DEPOSIT_ADDRESSES.may_load(deps.storage, address)?;
        match address_amount {
            Some(_) => Ok(CanDepositResponse { can_deposit: true }),
            None => Ok(CanDepositResponse { can_deposit: false }),
        }
    }
    pub fn get_deposit_addresses(deps: Deps) -> StdResult<DepositAddressesResponse> {
        let addresses = DEPOSIT_ADDRESSES
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|item| {
                let (address, _) = item?;
                Ok(address)
            })
            .collect::<StdResult<Vec<Addr>>>()?;
        Ok(DepositAddressesResponse {
            addresses: addresses.iter().map(|x| x.to_string()).collect(),
        })
    }
}
