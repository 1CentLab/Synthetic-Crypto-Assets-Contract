use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Storage, StdResult};
use cw_storage_plus::{ Map};
use sca::mint::Asset;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssetState {
    pub asset: Asset,
    pub reserve: Uint128,
    pub system_debt: Uint128
}


impl AssetState {
    pub fn default() -> AssetState{
        let asset = Asset{ 
            oracle: String::new(),
            pair: String::new(),
            sca: String::new(),
            collateral: String::new(),
            mcr: Uint128::new(0),
            multiplier:  Uint128::new(0)
        };

        AssetState { asset:  asset, reserve: Uint128::new(0), system_debt: Uint128::new(0) }
    }
}


pub fn get_asset_state(storage: &dyn Storage, sca: String, collateral: String) -> AssetState {
    let key = (sca.as_str(), collateral.as_str());
    let result = ASSET.load(storage, key);

    match result {
        Ok(value) => value,
        Err(_)=> AssetState::default()
    }
}

pub fn set_asset_state(storage: &mut dyn Storage, asset_state: AssetState) ->  StdResult<()>{
    let sca = asset_state.asset.sca.as_str();
    let collateral = asset_state.asset.collateral.as_str();
    let key= (sca, collateral);
    ASSET.save(storage, key, &asset_state)
}

pub const ASSET: Map<(&str, &str), AssetState> = Map::new("asset_state");
