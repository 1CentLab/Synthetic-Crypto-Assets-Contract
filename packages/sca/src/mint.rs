use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
    pub oracle: String,
    pub pair: String,
    pub sca: String,
    pub collateral: String,
    pub mcr: Uint128,
    pub multiplier: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidatedMessage {
    pub asset: Asset,
    pub liquidated_amount: Uint128,
    pub system_debt: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub controller: String
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetState {},
    GetScaOraclePrice{},
    GetScaPoolReserve{},
    GetPosition {user: String},
    GetAllPositions{}
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetAsset {
        asset: Asset
    },

    OpenPosition {
        collateral_amount: Uint128,
        ratio: Uint128
        //todo: Option here: for short minting 
    },

    ClosePosition {
        sca_amount: Uint128
    },

    //get asset price and perform checking liquidation
    MassUpdate {
    }
}
