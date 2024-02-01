use cosmwasm_std::{Address, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

pub const deposit_addresses: Map<Uint64, Address> = Map::new("deposit_addresses");
pub const allowances: Map<(Address, Uint128), Uint64> = Map::new("allowances");
pub const deposit_address_id: Item<Uint64> = Item::new("deposit_address_id");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Address,
    pub name: String,
    pub total_amount: Uint128,
}
