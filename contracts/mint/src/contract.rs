#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,WasmMsg, QueryRequest, WasmQuery, CosmosMsg};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use sca::mint::{Asset, ExecuteMsg, InstantiateMsg, QueryMsg};
use sca::oracle::{QueryMsg as OracleQueryMsg, ScaPriceResponse};
use crate::state::{set_asset, get_asset, get_position, set_position, Position};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:mint";
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
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetAsset { asset } => try_set_asset(deps, asset),
        ExecuteMsg::OpenPosition {collateral_amount, ratio} => try_open_position(deps, _env, info, collateral_amount, ratio),
        ExecuteMsg::ClosePosition { sca_amount } => try_close_position(deps, _env, info, sca_amount)
    }
}


pub fn try_set_asset(deps: DepsMut, asset: Asset) -> Result<Response, ContractError> {
    set_asset(deps.storage, asset)?;

    Ok(Response::new().add_attributes(vec![
        ("Method", "try_set_asset")
    ]))
}

pub fn try_open_position(deps: DepsMut, env: Env, info: MessageInfo, collateral_amount: Uint128, ratio: Uint128) -> Result<Response, ContractError> {
    let asset = get_asset(deps.storage);    

    if ratio < asset.multiplier{
        return Err(ContractError::UnderCollateralized)
    }

    let sca_price = query_sca_oracle_price(deps.as_ref());
    let sca_amount = collateral_amount * asset.multiplier / ratio * sca_price.1 / sca_price.0; // collateral / (ratio * sca_price/usd)

    //transfer amount of collateral to contract 
    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.collateral,
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom{
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount: collateral_amount
        })?,
        funds: vec![],
    }));


    // mint amount out of sca_token to user 
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.sca,
        msg: to_binary(&Cw20ExecuteMsg::Mint{
            recipient: info.sender.to_string(),
            amount: sca_amount
        })?,
        funds: vec![],
    }));


    // increase the existing data
    let mut position = get_position(deps.storage,info.sender.to_string().clone());
    position.size += collateral_amount;
    position.debt += sca_amount;
    position.open_time = env.block.time.seconds();
    set_position(deps.storage, info.sender.to_string(), position)?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_open_position")
    ]))
}

pub fn try_close_position(deps: DepsMut, _env: Env, info: MessageInfo, sca_amount: Uint128) -> Result<Response, ContractError> {  
    let mut position = get_position(deps.storage, info.sender.to_string().clone());

    if sca_amount > position.debt{
        return Err(ContractError::OverPaid{});
    }  

    if sca_amount == Uint128::new(0){
        return Err(ContractError::InvalidAmount{});
    }

    //get the corresponing collateral 
    let corresponding_collateral = sca_amount * position.size / position.debt;

    let mut messages: Vec<CosmosMsg> = vec![];
    let asset = get_asset(deps.storage);

    //transfer corresponding collaterael back to the sender
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.collateral,
        msg: to_binary(&Cw20ExecuteMsg::Transfer{
            recipient: info.sender.to_string().clone(),
            amount: corresponding_collateral
        })?,
        funds: vec![],
    }));

    //burn sca amount from the sender
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.sca,
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom{
            owner: info.sender.to_string().clone(),
            amount: sca_amount
        })?,
        funds: vec![],
    }));
    

    //update colalteral 
    position.debt = position.debt - sca_amount;
    position.size = position.size - corresponding_collateral;
    set_position(deps.storage, info.sender.to_string(), position)?;


    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_close_position")
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)),
        QueryMsg::GetScaOraclePrice { } => to_binary(&query_sca_oracle_price(deps)),
        QueryMsg::GetPosition { user } =>  to_binary(&query_position(deps, user))
    }
}

fn query_state(deps: Deps) -> Asset {
    get_asset(deps.storage)
}

fn query_position(deps: Deps, user: String) -> Position {
    get_position(deps.storage, user)
}

fn query_sca_oracle_price(deps: Deps) -> (Uint128, Uint128) {
    let res = get_sca_oracle_price(deps);

    match res {
        Ok(value) => value,
        Err(_) => (Uint128::new(0),Uint128::new(0))
    }
}

fn get_sca_oracle_price(deps: Deps) -> Result<(Uint128, Uint128), ContractError> {
    let state = get_asset(deps.storage);

    let query_msg = OracleQueryMsg::GetPrice { sca: state.sca};

    let query_response: ScaPriceResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: state.oracle,
         msg: to_binary(&query_msg)?,
    }))?;
    

    Ok((query_response.price, query_response.multiplier))
}

