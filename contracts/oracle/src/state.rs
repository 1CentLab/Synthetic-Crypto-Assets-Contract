use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Storage, StdResult};
use cw_storage_plus::{Map, Item};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub multiplier: Uint128
}



pub fn get_state(storage: &dyn Storage) -> State {
    STATE.load(storage).unwrap()
}

pub fn get_multiplier(storage: &dyn Storage) -> Uint128 {
    let state = STATE.load(storage).unwrap();
    state.multiplier
}

pub fn set_multiplier(storage: &mut dyn Storage, value :Uint128) ->  StdResult<()>{
    let mut state = get_state(storage);
    state.multiplier = value;
    STATE.save(storage, &state)
}


pub fn get_price(storage: &dyn Storage, sca: String) -> Uint128 {
    let price = PRICES.load(storage, sca);
    price.unwrap()
} 

pub fn set_price(storage: &mut dyn Storage, sca: String, price: Uint128) -> StdResult<()>{
    PRICES.save(storage, sca, &price)
}

pub const STATE: Item<State> = Item::new("state");
pub const PRICES: Map<String, Uint128> = Map::new("prices");