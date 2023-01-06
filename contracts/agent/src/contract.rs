use std::fmt::format;
use std::str::FromStr;
use std::{vec};

// #[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, Response, StakingMsg, StdResult, Uint128, Uint64,
    Order, Coin, DistributionMsg, CosmosMsg, SubMsg, entry_point, WasmMsg, Reply,
    SubMsgResult, Empty,
};

use cw2::set_contract_version;
use cw_utils::{one_coin, PaymentError, Duration, parse_reply_instantiate_data,};


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg,  QueryMsg};
use crate::state::{STAKING, NFT, NFT_ID, CACHE_NFT, CacheNFT};
use crate::wasm_query::{get_cw721_update_metadata_msg, get_cw721_mint_msg, get_cw721_burn_msg, get_staking_bond_msg,
                        get_staking_unbond_msg, get_staking_claim_msg, get_nft_owner, get_nft_metadata , get_staking_bonded };
use nft::contract::{Metadata, Status};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-agent-angel";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_NFT_REPLY_ID: u64 = 0;
const INSTANTIATE_STAKING_REPLY_ID: u64 = 1;
const EXECUTE_NEW_BOND_STAKING_REPLY_ID: u64 = 2;
const EXECUTE_RE_BOND_STAKING_REPLY_ID: u64 = 3;
const EXECUTE_NEW_BOND_NFT_REPLY_ID: u64 = 4;
const EXECUTE_RE_BOND_NFT_REPLY_ID: u64 = 5;
const EXECUTE_UNBOND_NFT_REPLY_ID: u64 = 6;
const EXECUTE_UNBOND_STAKING_REPLY_ID: u64 = 7;

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

    // TODO: Choose this implementation or the commented one.
    let nft_msg= nft::contract::InstantiateMsg{
        name: "angel_staking_nft".to_string(), 
        symbol: "ASM".to_string(), 
        minter: env.contract.address.clone().into() 
    };

    // let nft_msg= cw721_base::msg::InstantiateMsg{ 
    //     name: "angel_staking_nft".to_string(), 
    //     symbol: "ASM".to_string(), 
    //     minter: env.contract.address.clone().into() 
    // };

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

    let nft_contract_addr = NFT.load(deps.storage)?;
    let staking_contract_addr= STAKING.load(deps.storage)?;
    let reply_key : u64;
    let nft_id_info: String;
    let wasm_msg = match nft_id {
        Some(nft_id) => {
            // Query the nft contract and the staking contract, get the current amount staked. See that they match.
            let owner = get_nft_owner(deps.as_ref(), nft_id.clone(), &nft_contract_addr)?;
            if owner != info.sender {
                return Err(ContractError::NotOwnerNFT {  })
            }

            // NFT must have Status::Bonding
            let mut extension = get_nft_metadata(deps.as_ref(), nft_id.clone(), &nft_contract_addr)?;
            if extension.status == Status::Unbonding {
                return Err(ContractError::UnbondingNFT {  })             
            }

             // Current implementation only lets the user bond one coin per nft. Denom/Amount can be restacked
             // No second coin will be added and extension.native.len() == 1
            if extension.native[0].denom != d_coin.denom{
                return Err(ContractError::OnlyOneNativeCoinPerNFT {  } )                 
            } 

            // This is reduntant but increases security of bugs in initial contract version. 
            // nft and staking contract must be aligned on the amount stored on the nft. 
            let extension_amount = extension.native[0].amount;
            let staking_bonded_amount = get_staking_bonded(deps.as_ref(), nft_id.clone(), &staking_contract_addr)?;
            if extension_amount != staking_bonded_amount {
                return Err(ContractError::NFTStakingMismatch { staking: staking_bonded_amount.to_string(), nft: extension_amount.to_string() } )                   
            }

            extension.native[0].amount = extension.native[0].amount.checked_add(d_coin.amount).unwrap();

            let nft_id_uint128 = Uint128::from_str(&nft_id)?;
            // Create a new metadata, adding the amount.
            nft_id_info = format!("Rebond nft_id {}", nft_id.clone());

            // Storing info to be used on the reply entry point
            let extension = Metadata { native: vec![d_coin], status: Status::Bonded };
            let cache_nft = CacheNFT { sender: info.sender, nft_id, extension };
            CACHE_NFT.save(deps.storage, &cache_nft )?;

            reply_key = EXECUTE_RE_BOND_STAKING_REPLY_ID;
            let bond_msg = staking::msg::ExecuteMsg::Bond { nft_id: nft_id_uint128 };
            WasmMsg::Execute {
                contract_addr: staking_contract_addr.into(),
                msg: to_binary(&bond_msg)?,
                funds: info.funds,
            }

        },
        None => {
            let current_nft_id = NFT_ID.load(deps.storage)?;
            nft_id_info = format!("Mint nft_id {}", current_nft_id.clone());
            NFT_ID.update(deps.storage, |nft_id| -> Result<_, ContractError> {
                Ok(nft_id + Uint128::from(1u128))
            })?;

            // Storing info to be used on the reply entry point
            let extension = Metadata { native: vec![d_coin], status: Status::Bonded };
            let cache_nft = CacheNFT { sender: info.sender, nft_id: current_nft_id.to_string(), extension };
            CACHE_NFT.save(deps.storage, &cache_nft )?;

            reply_key = EXECUTE_NEW_BOND_STAKING_REPLY_ID;
            let bond_msg = staking::msg::ExecuteMsg::Bond { nft_id: current_nft_id };
            WasmMsg::Execute {
                contract_addr: staking_contract_addr.into(),
                msg: to_binary(&bond_msg)?,
                funds: info.funds,
            }
        }
    };

    let submsg:SubMsg<Empty> = SubMsg::reply_always(wasm_msg, reply_key);

    Ok(Response::new()
        .add_attribute("action", "execute_bond")
        .add_attribute("nft_id_info", nft_id_info)
        .add_submessage(submsg)
    )
}

pub fn execute_unbond(deps: DepsMut, env:Env, info: MessageInfo, nft_id:String)-> Result<Response, ContractError>{
    
    todo!()
    // let submsg:SubMsg<Empty>;
    // Ok(Response::new()
    //     .add_attribute("action", "execute_unbond")
    //     .add_attribute("nft_id_info", nft_id)
    //     .add_submessage(submsg)
    // )
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

    let mut another_reply = false;
    let wasm_msg : WasmMsg;
    let reply_key: u64;
    let submsg:SubMsg<Empty> ;

    match (reply.clone().id, reply.clone().result) {
        (INSTANTIATE_NFT_REPLY_ID, SubMsgResult::Ok(_))=> {
            let res = parse_reply_instantiate_data(reply.clone()).unwrap();  
            let addr = deps.api.addr_validate(res.contract_address.clone().as_str())?;
            NFT.save(deps.storage, &addr.to_string())?;

            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
            )
        },
        (INSTANTIATE_NFT_REPLY_ID, SubMsgResult::Err(_))=> {
            return Err(ContractError::NFTContractNotInstantiated {  });
        },
        (INSTANTIATE_STAKING_REPLY_ID, SubMsgResult::Ok(_))=>{
            let res = parse_reply_instantiate_data(reply.clone()).unwrap();  
            let addr = deps.api.addr_validate(res.contract_address.clone().as_str())?;
            STAKING.save(deps.storage, &addr.to_string())?;

            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
            )
        },
        (INSTANTIATE_STAKING_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::StakingContractNotInstantiated {  })
        },
        (EXECUTE_NEW_BOND_NFT_REPLY_ID, SubMsgResult::Ok(_))=>{
            let todo = 1;

            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
            )
        },
        (EXECUTE_NEW_BOND_NFT_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::UnableMintNFT {  } )
        },
        (EXECUTE_RE_BOND_NFT_REPLY_ID, SubMsgResult::Ok(_))=>{
            let todo = 1;

            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
            )
        },
        (EXECUTE_RE_BOND_NFT_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::UnableUpdateNFTMetadata {  })
        },
        (EXECUTE_NEW_BOND_STAKING_REPLY_ID, SubMsgResult::Ok(_))=>{
            another_reply = true;
            let cache_nft = CACHE_NFT.load(deps.storage)?;
            reply_key = EXECUTE_NEW_BOND_NFT_REPLY_ID;
            wasm_msg = get_cw721_mint_msg(
                &cache_nft.sender, 
                cache_nft.nft_id, 
                None,cache_nft.extension, 
                &Addr::unchecked(NFT.load(deps.storage)?)
            )?;
            submsg= SubMsg::reply_always(wasm_msg, reply_key);

            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
                .add_submessage(submsg)
            )
        },
        (EXECUTE_NEW_BOND_STAKING_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::UnableToStakeBondNewNFT {  })
        },
        (EXECUTE_RE_BOND_STAKING_REPLY_ID, SubMsgResult::Ok(_))=>{
            another_reply = true;
            let cache_nft = CACHE_NFT.load(deps.storage)?;
            reply_key = EXECUTE_RE_BOND_NFT_REPLY_ID;
            wasm_msg = get_cw721_update_metadata_msg(
                cache_nft.nft_id, 
                None, cache_nft.extension, 
                &Addr::unchecked(NFT.load(deps.storage)?)
            )?;
            submsg= SubMsg::reply_always(wasm_msg, reply_key);  
            
            Ok(Response::new()
                .add_attribute("action", "reply_handled")
                .add_attribute("reply_id", reply.id.to_string())
                .add_submessage(submsg)
            )
        },
        (EXECUTE_RE_BOND_STAKING_REPLY_ID, SubMsgResult::Err(_))=>{
            return Err(ContractError::UnableToStakeReBondNFT {  })
        },
        (_ , _) => {
            return Err(ContractError::UnknownReplyIdSubMsgResult { id: reply.id.to_string() });      
        },
      }

    //  if another_reply {
    //     Ok(Response::new()
    //         .add_attribute("action", "reply_handled")
    //         .add_attribute("reply_id", reply.id.to_string())
    //         .add_submessage(submsg)
    //     )
    //  } else {
    //     Ok(Response::new()
    //         .add_attribute("action", "reply_handled")
    //         .add_attribute("reply_id", reply.id.to_string())
    //     )
    //  } 
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