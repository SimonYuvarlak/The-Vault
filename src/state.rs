use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub name: String,
    pub total_amount: Uint128,
    pub expected_denom: String,
}

pub const deposit_addresses: Map<Addr, Uint128> = Map::new("deposit_addresses");
pub const allowances: Map<Addr, Uint128> = Map::new("allowances");
pub const state: Item<State> = Item::new("state");
