#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, from_binary};
use cw2::set_contract_version;
use cw20::{Cw20ReceiveMsg};

use crate::error::ContractError;
use sca::{controller::{ExecuteMsg, InstantiateMsg, QueryMsg}, mint::{Asset, LiquidatedMessage}};
use crate::state::{
    AssetState,
    get_asset_state, set_asset_state
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:controller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
   
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(message) => cw20_receiver_handler(deps, message),
        ExecuteMsg::AddAsset { asset } => try_add_asset(deps, asset)
    }
}

fn try_add_asset(deps:DepsMut, asset: Asset) -> Result<Response, ContractError>{
    let asset_state = AssetState{ 
        asset: asset,
        reserve: Uint128::new(0),
        system_debt: Uint128::new(0)
    };

    set_asset_state(deps.storage, asset_state)?;

    Ok(Response::new()
    .add_attribute("method", "add_asset_state"))
}

fn cw20_receiver_handler(deps: DepsMut, message: Cw20ReceiveMsg)-> Result<Response, ContractError> {
   match from_binary(&message.msg) {
       Ok::<LiquidatedMessage, _> (liq) =>{
           let asset = liq.asset;
           let mut asset_state = get_asset_state(deps.storage, asset.sca, asset.collateral);

            asset_state.reserve += liq.liquidated_amount;
            asset_state.system_debt += liq.system_debt;

            set_asset_state(deps.storage, asset_state)?;
            Ok(Response::new()
            .add_attribute("method", "receiver")
            )
       }
       Err(_) => return Err(ContractError::Cw20ReceiveError {})
   }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAssetState{sca, collateral} => to_binary(&query_asset_state(deps, sca, collateral)),
    }
}


fn query_asset_state(deps: Deps, sca: String, collateral: String) -> AssetState {
    get_asset_state(deps.storage, sca, collateral)
}

