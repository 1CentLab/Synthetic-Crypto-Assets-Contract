#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,CosmosMsg,WasmMsg, QueryRequest, WasmQuery};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, BalanceResponse, TokenInfoResponse};
use crate::error::ContractError;
use sca::pair::{ExecuteMsg, InstantiateMsg, QueryMsg, ReserveResponse};
use crate::state::{
    State, STATE, Reserve, RESERVES,
    get_tokens, get_lp_token,
    get_reserves, set_reserves, set_lp_token, get_fee
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pair";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        lp_token: String::from("not setted"),
        token0: msg.token0,
        token1: msg.token1,
        fee: msg.fee,
        multiplier: Uint128::new(1000)
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    
    let reserve = Reserve {
        reserve0: Uint128::new(0),
        reserve1: Uint128::new(0)
    };
    RESERVES.save(deps.storage, &reserve)?;

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
        //todo: Automatically set lp token
        ExecuteMsg::SetLpToken { lp_token } => try_set_lp_tokens(deps,lp_token),
        ExecuteMsg::AddLiquid {amount0, amount1} => try_add_liquid(deps, info, _env, amount0, amount1),
        ExecuteMsg::RemoveLiquid {liquid} => try_remove_liquid(deps, info, liquid),
        ExecuteMsg::Swap { amount_in , path} => try_swap(deps,info, _env, amount_in, path),
    }
}

pub fn try_set_lp_tokens(deps:DepsMut, lp_token: String) -> Result<Response, ContractError>{
    //todo: check authentication
    set_lp_token(deps.storage, lp_token)?;

    Ok(Response::new()
        .add_attribute("method", "set_lptoken")
    )
}

pub fn try_add_liquid(deps: DepsMut, info: MessageInfo, env: Env, amount0: Uint128, amount1: Uint128) -> Result<Response, ContractError> {
    let (token0, token1) = get_tokens(deps.storage);

    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token0,
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom{
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount: amount0
        })?,
        funds: vec![],
    }));

    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token1,
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom{
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount: amount1
        })?,
        funds: vec![],
    }));

    //calculate how much user contribute to the pool
    let multiplier = Uint128::new(10000000000);
    let (reserve0, reserve1) = get_reserves(deps.storage);

    let mut percent = multiplier;
    if reserve0 != Uint128::new(0) && reserve1 != Uint128::new(0){
        let percent0 = amount0 * multiplier / reserve0;
        let percent1 = amount1 * multiplier/ reserve1;
        percent = if percent0 < percent1 {percent0} else {percent1};
    }
   
    //todo:  if percent == 0 => contract error 

    //update reserves
    set_reserves(deps.storage, reserve0 + amount0, reserve1 + amount1)?;

    // mint LP_token for provider
    let (_, mut total_supply, _) = query_lp_token_info(deps.as_ref(), info.sender.to_string().clone());
    if total_supply == Uint128::new(0){
        total_supply = Uint128::new(100);
    }

    let lp_amount = percent * total_supply / multiplier;
    let lp_token = get_lp_token(deps.storage);
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lp_token,
        msg: to_binary(&Cw20ExecuteMsg::Mint{
            recipient: info.sender.to_string(),
            amount: lp_amount
        })?,
        funds: vec![],
    }));

    //msg 
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "add_liquidity")
    ]))
}

pub fn try_remove_liquid(deps: DepsMut, info: MessageInfo, liquid: Uint128) -> Result<Response, ContractError> {
    let (_, total_supply, _) = query_lp_token_info(deps.as_ref(), info.sender.to_string().clone());

    let (reserve0, reserve1) = get_reserves(deps.storage);
    let multiplier = Uint128::new(10000000000);
    let percent = liquid * multiplier / total_supply;

    //todo: if total supply = 0 -> contract error

    let token0_amount = reserve0 * percent / multiplier;
    let token1_amount = reserve1 * percent / multiplier;

    let mut messages: Vec<CosmosMsg> = vec![];

    let (token0, token1) = get_tokens(deps.storage);

    let sender =  info.sender.into_string();
    //transfer token0 and token1 amount back to owner 
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token0,
        msg: to_binary(&Cw20ExecuteMsg::Transfer { 
            recipient: sender.clone(), 
            amount: token0_amount 
        })?,
        funds: vec![],
    }));

    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token1,
        msg: to_binary(&Cw20ExecuteMsg::Transfer { 
            recipient: sender.clone(), 
            amount: token0_amount 
        })?,
        funds: vec![],
    }));

    //burn lp
    let lp_token = get_lp_token(deps.storage);
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lp_token,
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom { 
            owner: sender.clone(), 
            amount: liquid 
        })?,
        funds: vec![],
    }));

    //update reserve
    set_reserves(deps.storage, reserve0 - token0_amount, reserve1 - token1_amount)?;

    //msg 
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "remove_liquidity")
    ]))

}


// buy SCAs: token1 in => get token0
pub fn try_swap(deps: DepsMut,info: MessageInfo, env: Env,amount_in: Uint128, path: Vec<String>) -> Result<Response, ContractError> {
    let (token0, token1) = get_tokens(deps.storage);
    let (_, amount_out) = query_amounts_out(deps.as_ref(), amount_in, path.clone());

    //transfer amount in of token in from sender to contract 
    let mut messages: Vec<CosmosMsg> = vec![];
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: path[0].to_string(),
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom{
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount: amount_in
        })?,
        funds: vec![],
    }));


    // transfer amount out of token out to contract 
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: path[1].to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer{
            recipient: info.sender.to_string(),
            amount: amount_out
        })?,
        funds: vec![],
    }));

    //update reserve 
    let (reserve0, reserve1) = get_reserves(deps.storage);
    let (fee, multiplier) = get_fee(deps.storage);
    let amount_in_with_fee = amount_in * (multiplier - fee) / multiplier;
    
    if path.get(0).unwrap() == &token0 && path.get(1).unwrap() == &token1{
        // token in = token 0
        set_reserves(deps.storage, reserve0 + amount_in_with_fee, reserve1 - amount_out)?;
      
    }
    else if path.get(0).unwrap() == &token1 && path.get(1).unwrap() == &token0 {
        set_reserves(deps.storage, reserve0 - amount_out, reserve1 + amount_in_with_fee)?;
    }
    else{
       return Err( ContractError::InvalidPath {  });
    }

    //msg 
    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "swap")
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLpTokenInfo {user} => to_binary(&query_lp_token_info(deps, user)),
        QueryMsg::GetReserves {} => to_binary(&query_reserves(deps)),
        QueryMsg::GetAmountsOut { amount_in, path } => to_binary(&query_amounts_out(deps, amount_in, path)) 
    }
}

fn query_amounts_out(deps: Deps, amount_in: Uint128, path: Vec<String>) -> (Uint128, Uint128){
    let (token0, token1) = get_tokens(deps.storage);
    let (fee, multiplier) = get_fee(deps.storage);
    let (reserve0, reserve1) = get_reserves(deps.storage);

    let amount_in_with_fee = amount_in * (multiplier - fee);

    let reserve_in;
    let reserve_out;

    if path.get(0).unwrap() == &token0 && path.get(1).unwrap() == &token1{
        // token in = token 0
        reserve_in = reserve0;
        reserve_out = reserve1;   
    }
    else if path.get(0).unwrap() == &token1 && path.get(1).unwrap() == &token0 {
        reserve_in = reserve1;
        reserve_out = reserve0;
    }
    else{
        return (Uint128::new(0), Uint128::new(0))
    }
    
    let numurator=  amount_in_with_fee * reserve_out;
    let denominator = reserve_in * multiplier + amount_in_with_fee;

    let amount_out = numurator / denominator;

    
    return (amount_in, amount_out)
}

fn query_reserves(deps: Deps) -> ReserveResponse {
    let reserves = get_reserves(deps.storage);
    ReserveResponse { reserve0: reserves.0, reserve1: reserves.1 }
}

fn query_lp_token_info(deps: Deps, user: String) -> (Uint128, Uint128, u8){
    let balance = get_lp_token_info(deps, user);
    
    match balance {
        Ok(value) => value,
        Err(_) => (Uint128::new(0),Uint128::new(0), 0)
    }
}



fn get_lp_token_info(deps: Deps, user: String) -> Result<(Uint128, Uint128, u8), ContractError> {
    let query_msg = Cw20QueryMsg::Balance { address: user };

    let lp_token = get_lp_token(deps.storage);

    let query_response: BalanceResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: lp_token.clone(),
         msg: to_binary(&query_msg)?,
    }))?;
    

    let query_info_msg = Cw20QueryMsg::TokenInfo {  };
    let query_info_response: TokenInfoResponse =   deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: lp_token,
        msg: to_binary(&query_info_msg)?,
   }))?;

    Ok((query_response.balance, query_info_response.total_supply, query_info_response.decimals))
}

