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
use crate::state::{STAKING, NFT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-agent-angel";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_NFT_REPLY_ID: u64 = 0;
const INSTANTIATE_STAKING_REPLY_ID: u64 = 1;

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
        ExecuteMsg::Bond { nft_id } => unimplemented!(),
        ExecuteMsg::Unbond { nft_id } => unimplemented!(),
        ExecuteMsg::Claim { nft_id } => unimplemented!(),
    }
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