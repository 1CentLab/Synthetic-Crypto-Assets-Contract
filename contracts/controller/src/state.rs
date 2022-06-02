use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Storage, StdResult};
use cw_storage_plus::Item;
use sca::mint::Asset;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub reserve: Uint128
}



pub fn get_asset(storage: &dyn Storage) -> Asset {
    ASSET.load(storage).unwrap()
}


pub fn set_asset(storage: &mut dyn Storage, asset: Asset) ->  StdResult<()>{
    ASSET.save(storage, &asset)
}


pub fn get_state(storage: &dyn Storage) -> State {
    STATE.load(storage).unwrap()
}


pub fn set_state(storage: &mut dyn Storage, state: State) ->  StdResult<()>{
    STATE.save(storage, &state)
}


pub const STATE: Item<State> = Item::new("state");
pub const ASSET: Item<Asset> = Item::new("asset");
