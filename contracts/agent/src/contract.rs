use std::{vec};

// #[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, Response, StakingMsg, StdResult, Uint128, Uint64,
    Order, Coin, DistributionMsg, CosmosMsg, SubMsg, entry_point, WasmMsg, Reply,
    SubMsgResult
};

use cw2::set_contract_version;
use cw_utils::{one_coin, PaymentError, Duration, parse_reply_instantiate_data,};


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg,  QueryMsg};
use crate::state::{STAKING, NFT, NFT_ID};
use crate::cosmosmsg::{get_cw721_update_metadata_msg,get_cw721_mint_msg,get_cw721_burn_msg,get_staking_bond_msg,get_staking_unbond_msg, get_staking_claim_msg};
use nft::contract::{Metadata, Status};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-agent-angel";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_NFT_REPLY_ID: u64 = 0;
const INSTANTIATE_STAKING_REPLY_ID: u64 = 1;
const EXECUTE_NEW_BOND_NFT_REPLY_ID: u64 = 2;
const EXECUTE_RE_BOND_NFT_REPLY_ID: u64 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&msg.admin)?;
    deps.api.addr_validate(&msg.manager)?;
    deps.api.addr_validate(&msg.treasury)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    NFT_ID.save(deps.storage, &Uint128::zero())?;

    let nft_msg= nft::msg::InstantiateMsg{ 
        name: "angel_staking_nft".to_string(), 
        symbol: "ASM".to_string(), 
        minter: env.contract.address.clone().into() 
    };
   let instantiate_nft_msg = WasmMsg::Instantiate {
       code_id: msg.nft_code_id,
       funds: vec![],
       admin: Some(msg.admin.clone()),
       label: "angel_staking_nft".to_string(),
       msg: to_binary(&nft_msg)?,
   };
   let reply_msg_nft = SubMsg::reply_on_success(instantiate_nft_msg, INSTANTIATE_NFT_REPLY_ID);

   let staking_msg= staking::msg::InstantiateMsg{
    agent:env.contract.address.into(), 
    manager: msg.manager, 
    treasury: msg.treasury };
    let instantiate_staking_msg = WasmMsg::Instantiate {
        code_id: msg.staking_code_id,
        funds: vec![],
        admin: Some(msg.admin),
        label: "angel_staking".to_string(),
        msg: to_binary(&staking_msg)?,
    };
   let reply_msg_staking = SubMsg::reply_on_success(instantiate_staking_msg, INSTANTIATE_STAKING_REPLY_ID);
   
   Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_submessage(reply_msg_nft)
        .add_submessage(reply_msg_staking)
    )   
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond { nft_id } => execute_bond(deps, env, info, nft_id),
        ExecuteMsg::Unbond { nft_id } => execute_unbond(deps, env, info, nft_id),
        ExecuteMsg::Claim { nft_id } => execute_claim(deps, env, info, nft_id),
    }
}

// pub fn get_cw721_mint_msg(
//     owner: &Addr,
//     token_id: String,
//     token_uri: Option<String>,
//     extension: Metadata,
//     nft_contract_address: &Addr

// pub enum Status {
//     Bonded, Unbonding
// }

// pub struct Metadata {
//     pub native: Vec<Coin>,
//     pub status: Status,
// }
pub fn execute_bond (deps: DepsMut, env: Env, info: MessageInfo, nft_id: Option<String>) -> Result<Response, ContractError>{

    let d_coin = match one_coin(&info) {
        Ok(coin) => coin,
        Err(err) => {
            match err {
                PaymentError::NoFunds{} => {return Err(ContractError::NoFunds {  });}
                PaymentError::MultipleDenoms{} => {return Err(ContractError::MultipleDenoms {  });}
                _ => {return Err(ContractError::InvalidCoin {  });}
            }
        },
    };

    let nft_contract_address = NFT.load(deps.storage)?;

    let cosmos_msg = match nft_id {
        Some(nft_id) => {},
        None => {
            let current_nft_id = NFT_ID.load(deps.storage)?;
            NFT_ID.update(deps.storage, |mut nft_id| -> Result<_, ContractError> {
                Ok(nft_id + Uint128::from(1))
            })?;
            let extension = Metadata { native: vec![d_coin], status: Status::Bonded };
            get_cw721_mint_msg(&info.sender, current_nft_id.to_string(), None,extension, &Addr::unchecked(nft_contract_address));
        }
    };

    let msg = 
    let reply_msg_nft = SubMsg::reply_always(cosmos_msg, EXECUTE_NEW_BOND_NFT_REPLY_ID);
    Ok(Response::new())
}

pub fn execute_unbond(deps: DepsMut, env:Env, info: MessageInfo, nft_id:String)-> Result<Response, ContractError>{
    Ok(Response::new()) 
}

pub fn execute_claim(deps: DepsMut, env:Env, info: MessageInfo, nft_id:String)-> Result<Response, ContractError>{
    Ok(Response::new()) 
}






#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNFTAdress {  } => unimplemented!(),
        QueryMsg::GetStakingAdress {  } => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {

    match (reply.clone().id, reply.clone().result) {
        (INSTANTIATE_NFT_REPLY_ID, SubMsgResult::Ok(_))=> {
            let res = parse_reply_instantiate_data(reply.clone()).unwrap();  
            let addr = deps.api.addr_validate(res.contract_address.clone().as_str())?;
            NFT.save(deps.storage, &addr.to_string())?;
        },
        (INSTANTIATE_NFT_REPLY_ID, SubMsgResult::Err(_))=> {
            return Err(ContractError::NFTContractNotInstantiated {  });
        },
        (INSTANTIATE_STAKING_REPLY_ID, SubMsgResult::Ok(_))=>{
            let res = parse_reply_instantiate_data(reply.clone()).unwrap();  
            let addr = deps.api.addr_validate(res.contract_address.clone().as_str())?;
            STAKING.save(deps.storage, &addr.to_string())?;
        },
        (INSTANTIATE_STAKING_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::StakingContractNotInstantiated {  })
        },
        (_ , _) => {
            return Err(ContractError::UnknownReplyIdSubMsgResult { id: reply.id.to_string() });      
        },
      };
     Ok(Response::new()
    .add_attribute("action", "reply_handled")
    .add_attribute("reply_id", reply.id.to_string())
    )
  
}




#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{
        coins, Coin, CosmosMsg, Decimal, FullDelegation, Validator, from_binary, Delegation, StdError, attr
    };
 
}