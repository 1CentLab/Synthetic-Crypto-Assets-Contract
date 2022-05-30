use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Uint128, Storage, StdResult, Order};
use cw_storage_plus::{Map, Item};
use sca::mint::Asset;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Position {
    pub open_time: u64,
    pub size: Uint128,
    pub debt: Uint128,
    pub unrealized_liquidated_amount: Uint128,
    pub is_liquidated: bool
}

impl Position {
    pub fn default() -> Position {
        Position { open_time: 0, size: Uint128::new(0), debt: Uint128::new(0), unrealized_liquidated_amount: Uint128::new(0), is_liquidated: false}
    }
}

//todo: Seizing collateral


pub fn get_asset(storage: &dyn Storage) -> Asset {
    ASSET.load(storage).unwrap()
}


pub fn set_asset(storage: &mut dyn Storage, asset: Asset) ->  StdResult<()>{
    ASSET.save(storage, &asset)
}

pub fn get_position(storage: &dyn Storage, user: String) -> Position {
    let position = POSITION.load(storage, user);

    match position {
        Ok(value) => value,
        Err(_) => Position::default()
    }
}


pub fn get_all_positions(storage: &dyn Storage) -> Vec<String>{
    let mut vec = Vec::new();

    let data: StdResult<Vec<(Vec<u8>,Position)>>  = POSITION
        .range(storage, Option::None, Option::None, Order::Ascending)
        .collect();

    match data{
        Ok(positions)=>{
            for position in positions.iter(){
                let string_value = String::from_utf8(position.0.clone());
                match string_value {
                    Ok(s)=> vec.push(s.parse::<String>().unwrap()),
                    Err(_)=> continue
                }
            }
            vec
        }
        Err(_) => vec,
        } 
} 

pub fn set_position(storage: &mut dyn Storage, user: String, position: Position) -> StdResult<()> {
    POSITION.save(storage,user, &position)
}

pub const ASSET: Item<Asset> = Item::new("state");
pub const POSITION: Map<String, Position> = Map::new("position");