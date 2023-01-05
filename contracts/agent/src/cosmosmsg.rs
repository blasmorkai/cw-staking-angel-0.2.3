
use cosmwasm_std::{
   coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env,
   MessageInfo, QuerierWrapper, Response, StakingMsg, StdResult, Uint128, Uint64,
   Order, Coin, DistributionMsg, CosmosMsg, SubMsg, entry_point, WasmMsg, Reply,
   SubMsgResult
};
use cw721_base::MintMsg;
use cw_utils::Duration;
use nft::contract::{Metadata, Status};
use staking::msg::{ExecuteMsg, QueryMsg};


// pub enum Status {
//     Bonded, Unbonding
// }

// pub struct Metadata {
//     pub native: Option<Vec<Coin>>,
//     pub status: Status,
// }


pub fn get_cw721_mint_msg(
    owner: &Addr,
    token_id: String,
    token_uri: Option<String>,
    extension: Metadata,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    // create mint msg
    let mint_msg = nft::msg::ExecuteMsg::Mint(MintMsg { token_id, owner:owner.into(), token_uri, extension });
    let exec_mint = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    };
    let mint_cosmos_msg: CosmosMsg = exec_mint.into();
    Ok(mint_cosmos_msg)
 }
 
//  UpdateMetadata { token_id: String, token_uri: String, metadata: Metadata },

 pub fn get_cw721_update_metadata_msg(
    token_id: String,
    token_uri: Option<String>,
    extension: Metadata,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    let update_msg = nft::msg::ExecuteMsg::UpdateMetadata { token_id, token_uri, extension }; 
    let exec_update = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&update_msg)?,
        funds: vec![],
    };
    let update_cosmos_msg: CosmosMsg = exec_update.into();
    Ok(update_cosmos_msg)
 }
//     Burn { token_id: String },

pub fn get_cw721_burn_msg(
    token_id: String,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    let burn_msg = nft::msg::ExecuteMsg::Burn { token_id }; 
    let exec_burn = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    };
    let burn_cosmos_msg: CosmosMsg = exec_burn.into();
    Ok(burn_cosmos_msg)
 }


pub fn get_staking_bond_msg(
   nft_id:Uint128,
   staking_contract_address: &Addr
) -> StdResult<CosmosMsg> {
   let bond_msg = staking::msg::ExecuteMsg::Bond { nft_id };
   let exec_bond = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&bond_msg)?,
       funds: vec![],
   };
   let bond_cosmos_msg: CosmosMsg = exec_bond.into();
   Ok(bond_cosmos_msg)
}

pub fn get_staking_unbond_msg(
   nft_id: Uint128,
   amount: Uint128,
   staking_contract_address: &Addr,
) -> StdResult<CosmosMsg> {
   let unbond_msg = staking::msg::ExecuteMsg::Unbond { nft_id, amount }; 
   let unbond_exec = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&unbond_msg)?,
       funds: vec![],
   };
   let unbond_cosmos_msg: CosmosMsg = unbond_exec.into();
   Ok(unbond_cosmos_msg)
}
//    Claim {nft_id: Uint128 , sender: String, amount:Uint128},
pub fn get_staking_claim_msg(
   nft_id: Uint128,
   sender: &Addr,
   amount: Uint128,
   staking_contract_address: &Addr,
) -> StdResult<CosmosMsg> {
   let claim_msg = staking::msg::ExecuteMsg::Claim { nft_id, sender:sender.into(), amount }; 
   let claim_exec = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&claim_msg)?,
       funds: vec![],
   };
   let claim_cosmos_msg: CosmosMsg = claim_exec.into();
   Ok(claim_cosmos_msg)
}


// pub fn get_add_validator_msg(
//    address: String,
//    bond_denom: String,
//    unbonding_period: Duration,
//    staking_contract_address: &Addr,
// ) -> StdResult<CosmosMsg> {
//    // TODO: when we know how to query the chain we will remove the unbonding period parameter
//    let add_validator_msg = staking::msg::ExecuteMsg::AddValidator { address:address.into(), bond_denom, unbonding_period }; 
//    let add_validator_exec = WasmMsg::Execute {
//        contract_addr: staking_contract_address.into(),
//        msg: to_binary(&add_validator_msg)?,
//        funds: vec![],
//    };
//    let add_validator_cosmos_msg: CosmosMsg = add_validator_exec.into();
//    Ok(add_validator_cosmos_msg)
// }

// pub fn get_remove_validator_msg(
//    address: String,
//    staking_contract_address: &Addr,
// ) -> StdResult<CosmosMsg> {
   
//    let remove_validator_msg = staking::msg::ExecuteMsg::RemoveValidator { address: address.into() };
//    let remove_validator_exec = WasmMsg::Execute {
//        contract_addr: staking_contract_address.into(),
//        msg: to_binary(&remove_validator_msg)?,
//        funds: vec![],
//    };
//    let remove_validator_cosmos_msg: CosmosMsg = remove_validator_exec.into();
//    Ok(remove_validator_cosmos_msg)
// }