use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Storage, StdResult};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub lp_token: String,
    pub token0: String,   
    pub token1: String,  // for simplifcity, token1 represent usd 
    pub fee: Uint128,
    pub multiplier: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reserve {
    pub reserve0: Uint128,
    pub reserve1: Uint128,
}

pub fn get_state(storage: &dyn Storage) -> State {
    STATE.load(storage).unwrap()
}

pub fn set_lp_token(storage: &mut dyn Storage, lp_token: String) -> StdResult<()>{
    let mut state = get_state(storage);
    state.lp_token = lp_token;
    STATE.save(storage, &state)
}

pub fn get_lp_token(storage: &dyn Storage) -> String {
    let state = STATE.load(storage).unwrap();
    state.lp_token
}

pub fn get_tokens(storage: &dyn Storage) -> (String, String){
    let state = STATE.load(storage).unwrap();
    (state.token0, state.token1)
} 

pub fn get_reserves(storage: &dyn Storage) -> (Uint128, Uint128){
    let state = RESERVES.load(storage).unwrap();
    (state.reserve0, state.reserve1)
}

pub fn set_reserves(storage: &mut dyn Storage, new_reserve0: Uint128, new_reserve1: Uint128) -> StdResult<()>{
    let new_reserve = Reserve{
        reserve0: new_reserve0,
        reserve1: new_reserve1
    };
   
   
    RESERVES.save(storage, &new_reserve)
}


pub fn get_fee(storage: &dyn Storage) -> (Uint128, Uint128){
    let state = get_state(storage);
    (state.fee, state.multiplier)
}

pub const STATE: Item<State> = Item::new("state");
pub const RESERVES: Item<Reserve> = Item::new("reserves");
