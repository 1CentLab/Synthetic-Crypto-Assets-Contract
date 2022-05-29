#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use sca::oracle::{ScaPriceResponse,ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, get_price, get_multiplier, set_price};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        multiplier: msg.multiplier
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

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
        ExecuteMsg::SetPrice { sca, price} => try_set_price(deps, sca, price),
    }
}

pub fn try_set_price(deps: DepsMut, sca: String, price: Uint128) -> Result<Response, ContractError> {
    set_price(deps.storage, sca, price)?;

    Ok(Response::new().add_attribute("method", "set_price"))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice {sca} => to_binary(&query_price(deps, sca)),
    
    }
}

fn query_price(deps: Deps, sca:String) -> ScaPriceResponse {
    let price = get_price(deps.storage, sca);
    let multiplier = get_multiplier(deps.storage);
    ScaPriceResponse { price: price, multiplier: multiplier }
}