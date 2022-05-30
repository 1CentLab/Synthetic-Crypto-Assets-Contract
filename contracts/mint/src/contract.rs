#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,WasmMsg, QueryRequest, WasmQuery, CosmosMsg};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use sca::mint::{Asset, ExecuteMsg, InstantiateMsg, QueryMsg};
use sca::pair::{QueryMsg as PoolQueryMsg, ReserveResponse};
use sca::oracle::{QueryMsg as OracleQueryMsg, ScaPriceResponse};
use crate::state::{set_asset, get_asset, 
    get_position, set_position, Position, get_all_positions};

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
        ExecuteMsg::ClosePosition { sca_amount } => try_close_position(deps, _env, info, sca_amount),
        ExecuteMsg::MassUpdate {  } => try_mass_update(deps, _env, info)
     }
}

pub fn try_mass_update(deps:DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    //get price
    let asset = get_asset(deps.storage);
    let sca_prices = get_sca_oracle_price(deps.as_ref());

    //get all position 
    let positions = get_all_positions(deps.storage);

    for p_user in positions{
        let position  = get_position(deps.storage, p_user);
    }


    //find position under collateralize 


    // force liquidation + auction opening (Check premium: Perform buy back or auction)


    let mut messages: Vec<CosmosMsg> = vec![];
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_open_position")
    ]))
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
    let sca_amount = collateral_amount * asset.multiplier / ratio * sca_price.price / sca_price.multiplier; // collateral / (ratio * sca_price/usd)

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




fn update_position(deps: DepsMut, p_user: String, sca_prices: (Uint128, Uint128), asset: Asset) -> Result<Response, ContractError>{
    let mut position = get_position(deps.storage, p_user);
    let off_chain_value = position.debt * asset.mcr * sca_prices.0 / sca_prices.1 / asset.multiplier;

    if off_chain_value < position.size {
        return Ok(Response::new().add_attribute("Method", "try_update_position"));
    }

    //amount of collateral amount need to be reduced from the supply
    let c_amount = off_chain_value - position.size;
    
    

    let mut messages: Vec<CosmosMsg> = vec![];
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_update_position")
    ]))

    // if discount ==> user -> sca -> contract, get back size of real world collateral
    // if premium ==> contract -> buy sca in pool
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&query_state(deps)),
        QueryMsg::GetScaOraclePrice { } => to_binary(&query_sca_oracle_price(deps)),
        QueryMsg::GetPosition { user } =>  to_binary(&query_position(deps, user)),
        QueryMsg::GetAllPositions {  } => to_binary(&query_all_position(deps)),
        QueryMsg::GetScaPoolReserve{  } => to_binary(&query_sca_pool_price(deps))
    }
}

fn query_all_position(deps:Deps) -> Vec<String>{
    get_all_positions(deps.storage)
}

fn query_state(deps: Deps) -> Asset {
    get_asset(deps.storage)
}

fn query_position(deps: Deps, user: String) -> Position {
    get_position(deps.storage, user)
}

fn query_sca_oracle_price(deps: Deps) -> ScaPriceResponse{
    let res = get_sca_oracle_price(deps);

    match res {
        Ok(value) => value,
        Err(_) => ScaPriceResponse {
            price: Uint128::new(0),
            multiplier: Uint128::new(0)
        }
    }
}

fn get_sca_oracle_price(deps: Deps) -> Result<ScaPriceResponse, ContractError> {
    let state = get_asset(deps.storage);

    let query_msg = OracleQueryMsg::GetPrice { sca: state.sca};

    let query_response: ScaPriceResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: state.oracle,
         msg: to_binary(&query_msg)?,
    }))?;
    

    Ok(query_response)
}

fn query_sca_pool_price(deps: Deps) -> ReserveResponse {
    let res = get_sca_pool_price(deps);

    match res {
        Ok(value) => value ,
        Err(_) => ReserveResponse{
            reserve0: Uint128::new(0),
            reserve1: Uint128::new(1)
        }
    }
}

fn get_sca_pool_price(deps: Deps) -> Result<ReserveResponse, ContractError> {
    let asset = get_asset(deps.storage);

    let query_msg = PoolQueryMsg::GetReserves {  };

    let query_response: ReserveResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: asset.pair,
         msg: to_binary(&query_msg)?,
    }))?;
    
    Ok(query_response)
}
