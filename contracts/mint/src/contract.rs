#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,WasmMsg, QueryRequest, WasmQuery, CosmosMsg};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};
use std::str;
use crate::error::ContractError;
use sca::mint::{Asset, LiquidatedMessage, ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use sca::pair::{QueryMsg as PoolQueryMsg, ReserveResponse};
use sca::oracle::{QueryMsg as OracleQueryMsg, ScaPriceResponse};
use crate::state::{
    CONTROLLER,
    set_asset, get_asset, 
    get_position, set_position, Position,get_all_positions,
    ClosedPosition, set_closed_position, remove_position
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:mint";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONTROLLER.save(deps.storage, &msg.controller)?;
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

pub fn try_mass_update(deps:DepsMut, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    //get all position 
    let positions = get_all_positions(deps.storage);
    let asset = get_asset(deps.storage);

    let mut liquidated_collateral = Uint128::new(0);
    let mut system_debt = Uint128::new(0);
    let mut unsufficent_collateral = Uint128::new(0);
    for p_user in positions{
        let  position = update_position(deps.as_ref(), p_user.clone(), &asset);
        
        liquidated_collateral += position.unrealized_liquidated_amount;
        system_debt += position.unrealized_system_debt;
        unsufficent_collateral += position.unrealized_unsuffcient_collateral;
        
        //close if position is being liqudiated 
        if position.is_liquidated {            
            // update closed position
            let closed_position = ClosedPosition{
                close_time: env.block.time.seconds(),
                size: position.initial_size,
                debt: position.initial_debt,
                is_liquidated: true
            };

            set_closed_position(deps.storage, p_user.clone(), closed_position)?;
            remove_position(deps.storage, p_user.clone());
        }


        set_position(deps.storage, p_user, position)?;
    }

    //transfer liquidated amount to controller contract
    let mut messages: Vec<CosmosMsg> = vec![];

    if liquidated_collateral == Uint128::new(0) {
        return Ok(Response::new().add_attribute("method", "try_mass_update"))
    }

    let liq_msg = _get_liquidated_msg(deps.as_ref(), asset, liquidated_collateral, system_debt, unsufficent_collateral)?;
    messages.push(liq_msg);

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("method", "try_mass_update")
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
    let sca_amount = collateral_amount * asset.multiplier / ratio * sca_price.multiplier/sca_price.price; // collateral / (ratio * sca_price/usd)

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
    position.initial_size += collateral_amount;
    position.initial_debt += sca_amount;
    position.size += collateral_amount;
    position.debt += sca_amount;
    position.open_time = env.block.time.seconds();
    position.entry_price = sca_price.price;
    position.multiplier = sca_price.multiplier;
    set_position(deps.storage, info.sender.to_string(), position)?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_open_position")
    ]))
}

pub fn try_close_position(deps: DepsMut, env: Env, info: MessageInfo, sca_amount: Uint128) -> Result<Response, ContractError> {  
    let mut position = get_position(deps.storage, info.sender.to_string().clone());
    
    if sca_amount > position.debt{
        return Err(ContractError::OverPaid{});
    }  

    if sca_amount == Uint128::new(0){
        return Err(ContractError::InvalidAmount{});
    }

    // update position
    let asset = get_asset(deps.storage);
    position = update_position(deps.as_ref(), info.sender.to_string().clone(), &asset);

    let receive_collateral;
    let liquidated_collateral;
    let system_debt;

    if position.is_liquidated {
        return Err(ContractError::LiquidatedPosition{});
    }
    else{
        //get the corresponing collateral 
        receive_collateral = position.size * sca_amount / position.debt;
        liquidated_collateral = position.unrealized_liquidated_amount * sca_amount / position.debt;
        system_debt = position.unrealized_system_debt * sca_amount / position.debt;
    }

    let mut messages: Vec<CosmosMsg> = vec![];
    //transfer corresponding collaterael back to the sender
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.collateral.clone(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer{
            recipient: info.sender.to_string().clone(),
            amount: receive_collateral
        })?,
        funds: vec![],
    }));

    //burn sca amount from the sender
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.sca.clone(),
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom{
            owner: info.sender.to_string().clone(),
            amount: sca_amount
        })?,
        funds: vec![],
    }));
    
    // if liquidated collateral > 0 ==> transfer liquidated collateral to controller
    if liquidated_collateral > Uint128::new(0){
        let liq_msg = _get_liquidated_msg(deps.as_ref(), asset.clone(), liquidated_collateral, system_debt, Uint128::new(0))?;
        messages.push(liq_msg);
    }

    //update colalteral 
    position.debt = position.debt - sca_amount;
    position.size = position.size - receive_collateral;

    if sca_amount == position.debt{
        let closed_position = ClosedPosition{
            close_time: env.block.time.seconds(),
            size: position.initial_size,
            debt: position.initial_debt,
            is_liquidated: false
        };

        set_closed_position(deps.storage, info.sender.clone().into_string(), closed_position)?;
        remove_position(deps.storage, info.sender.clone().into_string());
    }

    set_position(deps.storage, info.sender.to_string(), position)?;
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "try_close_position")
    ]))
}

fn update_position(deps: Deps, p_user: String, asset: &Asset) -> Position{
    let mut position = get_position(deps.storage, p_user.clone());

    let sca_oracle_price = query_sca_oracle_price(deps);
    let off_chain_value = position.debt * asset.mcr * sca_oracle_price.price / sca_oracle_price.multiplier / asset.multiplier;  // debt * price * MCR 

    if off_chain_value < position.size || position.is_liquidated == true {
        return position;
    }

    let numerator = (off_chain_value - position.size) * asset.multiplier;  // 3.7
    let denominator=  (asset.mcr * asset.premium_rate - (asset.multiplier * asset.multiplier)) / asset.multiplier;

    let liquidated_amount = numerator/denominator;
    let debt = liquidated_amount * asset.premium_rate /sca_oracle_price.price;   // 3.8

    position.unrealized_liquidated_amount= liquidated_amount;
    position.unrealized_system_debt = debt;

    if position.unrealized_liquidated_amount >= position.size {
        position.is_liquidated = true;

        position.unrealized_system_debt = position.debt;
        position.unrealized_liquidated_amount = position.unrealized_system_debt * sca_oracle_price.price * asset.premium_rate / asset.multiplier / sca_oracle_price.multiplier;

        position.unrealized_unsuffcient_collateral= position.unrealized_liquidated_amount - position.size;
        position.unrealized_liquidated_amount = position.size;
      
        
        position.size = Uint128::new(0);
        position.debt = Uint128::new(0);


    }
    else {
        position.size = position.size - position.unrealized_liquidated_amount;
        position.debt = position.debt - position.unrealized_system_debt;
    }

    position
}

fn _get_liquidated_msg(deps: Deps, asset: Asset, liquidated_amount: Uint128, system_debt: Uint128, unsufficent_collateral: Uint128) -> StdResult<CosmosMsg> {
    let collateral_token = asset.collateral.clone();
    let liqudated_message = LiquidatedMessage{
        asset: asset,
        liquidated_amount: liquidated_amount,
        system_debt: system_debt,
        unsufficent_amount: unsufficent_collateral
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collateral_token,
        msg: to_binary(&Cw20ExecuteMsg::Send{
            contract: CONTROLLER.load(deps.storage)?,
            amount: liquidated_amount,
            msg: to_binary(&liqudated_message)?
        })?,
        funds: vec![],
    });

    Ok(msg)
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
