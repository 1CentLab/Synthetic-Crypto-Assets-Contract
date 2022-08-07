#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, from_binary, QueryRequest, WasmQuery, CosmosMsg, WasmMsg};
use cw2::set_contract_version;
use cw20::{Cw20ReceiveMsg, Cw20ExecuteMsg};

use crate::error::ContractError;
use sca::{controller::{ExecuteMsg, InstantiateMsg, QueryMsg}, mint::{Asset, LiquidatedMessage}};
use sca::pair::{QueryMsg as PoolQueryMsg, ReserveResponse};
use sca::oracle::{QueryMsg as OracleQueryMsg, ScaPriceResponse, MigrateMsg};
use crate::state::{
    AssetState,
    get_asset_state, set_asset_state
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:controller";
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
        ExecuteMsg::Receive(message) => cw20_receiver_handler(deps, message),
        ExecuteMsg::AddAsset { asset } => try_add_asset(deps, asset),
        ExecuteMsg::BuyAuction { sca, collateral, sca_amount} => buy_auction(deps, info, sca, collateral, sca_amount)
    }
}

fn try_add_asset(deps:DepsMut, asset: Asset) -> Result<Response, ContractError>{
    let asset_state = AssetState{ 
        asset: asset,
        reserve: Uint128::new(0),
        system_debt: Uint128::new(0),
        unsufficent_amount: Uint128::new(0)
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
            asset_state.system_debt = asset_state.system_debt + liq.system_debt + liq.unsufficent_amount;
            asset_state.unsufficent_amount += liq.unsufficent_amount;

            set_asset_state(deps.storage, asset_state)?;
            Ok(Response::new()
                .add_attribute("method", "receiver")
                .add_attribute("origin", "from_mint")
            )
       }
       Err(_) => Ok(Response::new()
            .add_attribute("method", "receiver")
            .add_attribute("origin", "others")
        )
   }
}


fn buy_auction(deps: DepsMut, info: MessageInfo, sca: String, collateral: String, sca_amount: Uint128) -> Result<Response, ContractError> {
    let mut asset_state = get_asset_state(deps.storage, sca, collateral);
    let asset = asset_state.asset.clone();

    let oracle_price = query_sca_oracle_price(deps.as_ref(), asset.sca.clone(), asset.collateral.clone());
    let pool_reserves = query_sca_pool_price(deps.as_ref(), asset.sca.clone(),asset.collateral.clone());
   
    let on_price = pool_reserves.reserve1 * oracle_price.multiplier / pool_reserves.reserve0; // multiplier with oracle price multiplier to handel decimal case 
    
    // let off_price = oracle_price.price;

    // // if on_price < off_price ==> Buy from pool. Else: Do auction  
    // if on_price < off_price {
    //     //todo: buy from pool 
    //     return Err(ContractError::InPremium { })
    // }

    //Valiate sca_Amount of user's input 
    if sca_amount > asset_state.system_debt || sca_amount == Uint128::new(0){
        return Err(ContractError::InvalidAmount { });
    }

    // Calculate amount of collateral to pay for user 
    let expected_offer_collateral = sca_amount * on_price * asset.premium_rate / asset.multiplier / oracle_price.multiplier;
    if expected_offer_collateral > asset_state.reserve {
        return Err(ContractError::InsufficentReserve {});
    }

    asset_state.reserve -= expected_offer_collateral;
    asset_state.system_debt -= sca_amount;
    set_asset_state(deps.storage, asset_state)?;
    
    // ------ do auction at discount 
    let mut messages: Vec<CosmosMsg> = vec![];

    //burn sca amount from the sender
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.sca.clone(),
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom{
            owner: info.sender.to_string().clone(),
            amount: sca_amount
        })?,
        funds: vec![],
    }));

    //transfer expected offer collateral to user 
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: asset.collateral.clone(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer{
            recipient: info.sender.to_string().clone(),
            amount: expected_offer_collateral
        })?,
        funds: vec![],
    }));


    Ok(Response::new().add_messages(messages).add_attributes(vec![
        ("Method", "buy_auction")
    ]))

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAssetState{sca, collateral} => to_binary(&query_asset_state(deps, sca, collateral)),
        QueryMsg::GetScaOraclePrice {sca, collateral } => to_binary(&query_sca_oracle_price(deps, sca, collateral)),
        QueryMsg::GetScaPoolReserve { sca, collateral} => to_binary(&query_sca_pool_price(deps, sca, collateral))
    }
}


fn query_asset_state(deps: Deps, sca: String, collateral: String) -> AssetState {
    get_asset_state(deps.storage, sca, collateral)
}

fn query_sca_oracle_price(deps: Deps, sca: String, collateral: String) -> ScaPriceResponse{
    let asset = get_asset_state(deps.storage, sca, collateral);
    let res = get_sca_oracle_price(deps, asset.asset);

    match res {
        Ok(value) => value,
        Err(_) => ScaPriceResponse {
            price: Uint128::new(0),
            multiplier: Uint128::new(0)
        }
    }
}

fn get_sca_oracle_price(deps: Deps, asset: Asset) -> Result<ScaPriceResponse, ContractError> {

    let query_msg = OracleQueryMsg::GetPrice { sca: asset.sca};

    let query_response: ScaPriceResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: asset.oracle,
         msg: to_binary(&query_msg)?,
    }))?;
    

    Ok(query_response)
}

fn query_sca_pool_price(deps: Deps, sca: String, collateral: String) -> ReserveResponse {
    let asset_state = get_asset_state(deps.storage, sca, collateral);
    let res = get_sca_pool_price(deps, asset_state.asset);

    match res {
        Ok(value) => value ,
        Err(_) => ReserveResponse{
            reserve0: Uint128::new(0),
            reserve1: Uint128::new(1)
        }
    }
}

fn get_sca_pool_price(deps: Deps, asset: Asset) -> Result<ReserveResponse, ContractError> {
    let query_msg = PoolQueryMsg::GetReserves {  };

    let query_response: ReserveResponse =
      deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
         contract_addr: asset.pair,
         msg: to_binary(&query_msg)?,
    }))?;
    
    Ok(query_response)
}