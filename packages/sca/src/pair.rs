use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub token0: String,  
    pub token1: String,
    pub fee: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetLpToken{
        lp_token: String
    },

    // add liquid an receive liquid back 
    AddLiquid{
        amount0: Uint128,
        amount1: Uint128
    },

    RemoveLiquid {
        liquid: Uint128 
    },


    // swap 
    Swap {
        amount_in: Uint128,
        path: Vec<String>
    },

    // Collect 
    

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetLpTokenInfo {user: String},
    
    //prices 
    GetReserves {}, 

    //minimum received 
    GetAmountsOut {amount_in: Uint128, path: Vec<String>}
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReserveResponse {
    pub reserve0: Uint128,
    pub reserve1: Uint128
}
