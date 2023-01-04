use std::{vec};

// #[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, WasmMsg};
use cosmwasm_std::{
    coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env,
    MessageInfo, QuerierWrapper, Response, StakingMsg, StdResult, Uint128, Uint64,
    Order, Coin, DistributionMsg, CosmosMsg,
};

use cw2::set_contract_version;
use cw_utils::{one_coin, PaymentError, Duration};


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg,  QueryMsg};
use crate::state::{STAKING, NFT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-agent-angel";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&msg.admin)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let nft_msg= nft::msg::InstantiateMsg{ 
        name: "angel_staking_nft".to_string(), 
        symbol: "ASM".to_string(), 
        minter: env.contract.address.into() };

   let instantiate_nft_msg = WasmMsg::Instantiate {
       code_id: msg.nft_code_id,
       funds: vec![],
       admin: Some(msg.admin),
       label: "angel_staking_nft".to_string(),
       msg: to_binary(&nft_msg)?,
   };
    Ok(Response::new()
        .add_attribute("action", "instantiate")
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