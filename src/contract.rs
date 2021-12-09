#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,BankMsg, SubMsg, WasmMsg};
use cosmwasm_std::{Uint128, Addr,};

use cw2::set_contract_version;
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, query_balance, query_token_info,
};
use cw20_base::state::{MinterData, TokenInfo, TOKEN_INFO};



use crate::error::{ContractError, NotCW20, NoDepositedToken};
use crate::msg::{PriceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ Valut, VALUTS, Supply, TOTAL_SUPPLY, ORACEL_PRICE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:l-ust";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let price = msg.initial_price;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ORACEL_PRICE.save(deps.storage, &price)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
        .add_attribute("price", msg.initial_price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPrice { price } => trt_set_price(deps, price),
        ExecuteMsg::Mint(msg) => {
            try_mint(deps, env, deposit_amount, &info.sender)
        },
        ExecuteMsg::Redeem(msg) => {
            try_redeem(deps, env, &info.sender)
        },
        // inherited from cw20-base
        ExecuteMsg::Transfer { recipient, amount } => {
            Ok(execute_transfer(deps, env, info, recipient, amount)?)
        }
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => Ok(execute_send(deps, env, info, contract, amount, msg)?),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_increase_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => Ok(execute_decrease_allowance(
            deps, env, info, spender, amount, expires,
        )?),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => Ok(execute_transfer_from(
            deps, env, info, owner, recipient, amount,
        )?),
        ExecuteMsg::BurnFrom { owner, amount } => {
            Ok(execute_burn_from(deps, env, info, owner, amount)?)
        }
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => Ok(execute_send_from(
            deps, env, info, owner, contract, amount, msg,
        )?),
    }
}

pub fn trt_set_price(deps: DepsMut, price: Uint128) -> Result<Response, ContractError> {
    ORACEL_PRICE.save(deps.storage, &price)?;

    Ok(Response::new().add_attribute("method", "trt_set_price"))
}

pub fn try_mint(deps: DepsMut, env: Env, deposit_amount: Uint128, sender: &Addr) -> Result<Response, ContractError> {
    // send sLSRV to contract
    send_tokens(&sLSRVADDR, &env.contract.address, deposit_amount )?
    let price = ORACEL_PRICE.may_load(deps.storage)?
    // mock code
    // deposit ratio is 200%
    let mint_amount =  deposit_amount.clone() * price * 0.2;
    
    let sub_info = MessageInfo {
        sender: env.contract.address.clone(),
        funds: vec![],
    };

    execute_mint(deps, env, sub_info, sender.to_string(), mint_amount.clone())?;

    TOTAL_SUPPLY.update(deps.storage, |mut supply| -> Result<_, ContractError> {
        supply.deposited = supply.deposited + deposit_amount.clone();
        supply.issued = supply.issued + mint_amount.clone();
        Ok(supply)
    })?;

    let old_valut = Valut.may_load(deps.storage, sender)?.unwrap_or_default();
    let new_valut = old_valut.map(|v| Valut {
        deposited: v.deposited + deposit_amount,
        issued: v.issued + mint_amount
    })

    Valut.save(deps.storage, sender, &new_valut)?;


    Ok(Response::new().add_attribute("method", "try_mint"))
}

fn send_tokens(contract_address: &Addr, to: &Addr, balance: &Uint128) -> StdResult<CosmosMsg> {
    let msg = Cw20ExecuteMsg::Transfer {
        recipient: to.into(),
        amount: balance.into(),
    };
    Ok(WasmMsg::Execute {
            contract_addr: contract_address.into(),
            msg: to_binary(&msg)?,
            funds: vec![],
    }
    .into())
}

pub fn try_redeem(deps: DepsMut, env: Env, sender: &Addr)  -> Result<Response, ContractError> {
    if let Some(deposited, issued) = Valut.load(deps.storage, sender) {
        let valut = Valut {
            deposited: 0,
            issued: 0
        }
        Valut.save(deps.storage, sender, &valut)?;
        send_tokens(&sLSRVADDR, &sender, deposited)?
        execute_burn(deps.branch(), env.clone(), info.clone(), issued)?;
    } else {
        return Err(NoDepositedToken {})
    }

    Ok(Response::new().add_attribute("method", "try_redeem"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrice {} => to_binary(&query_price(deps)?),
        // inherited from cw20-base
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
    }
}

fn query_price(deps: Deps) -> StdResult<PriceResponse> {
    let price = ORACEL_PRICE.may_load(deps.storage)?.unwrap_or_default();
    Ok(PriceResponse { price: price })
}