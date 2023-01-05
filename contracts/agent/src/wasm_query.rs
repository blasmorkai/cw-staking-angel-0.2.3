
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
 ) -> StdResult<WasmMsg> {
    // create mint msg
    let mint_msg = nft::msg::ExecuteMsg::Mint(MintMsg { token_id, owner:owner.into(), token_uri, extension });
    let mint_wasm_msg = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    };
    Ok(mint_wasm_msg)
 }
 
//  UpdateMetadata { token_id: String, token_uri: String, metadata: Metadata },

 pub fn get_cw721_update_metadata_msg(
    token_id: String,
    token_uri: Option<String>,
    extension: Metadata,
    nft_contract_address: &Addr
 ) -> StdResult<WasmMsg> {
    let update_msg = nft::msg::ExecuteMsg::UpdateMetadata { token_id, token_uri, extension }; 
    let update_wasm_msg = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&update_msg)?,
        funds: vec![],
    };
    Ok(update_wasm_msg)
 }
//     Burn { token_id: String },

pub fn get_cw721_burn_msg(
    token_id: String,
    nft_contract_address: &Addr
 ) -> StdResult<WasmMsg> {
    let burn_msg = nft::msg::ExecuteMsg::Burn { token_id }; 
    let burn_wasm_msg = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    };
    Ok(burn_wasm_msg)
 }


pub fn get_staking_bond_msg(
   nft_id:Uint128,
   staking_contract_address: &Addr
) -> StdResult<WasmMsg> {
   let bond_msg = staking::msg::ExecuteMsg::Bond { nft_id };
   let bond_wasm_msg = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&bond_msg)?,
       funds: vec![],
   };
   Ok(bond_wasm_msg)
}

pub fn get_staking_unbond_msg(
   nft_id: Uint128,
   amount: Uint128,
   staking_contract_address: &Addr,
) -> StdResult<WasmMsg> {
   let unbond_msg = staking::msg::ExecuteMsg::Unbond { nft_id, amount }; 
   let unbond_wasm_msg = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&unbond_msg)?,
       funds: vec![],
   };
   Ok(unbond_wasm_msg)
}

//    Claim {nft_id: Uint128 , sender: String, amount:Uint128},
pub fn get_staking_claim_msg(
   nft_id: Uint128,
   sender: &Addr,
   amount: Uint128,
   staking_contract_address: &Addr,
) -> StdResult<WasmMsg> {
   let claim_msg = staking::msg::ExecuteMsg::Claim { nft_id, sender:sender.into(), amount }; 
   let claim_wasm_msg = WasmMsg::Execute {
       contract_addr: staking_contract_address.into(),
       msg: to_binary(&claim_msg)?,
       funds: vec![],
   };
   Ok(claim_wasm_msg)
}

pub fn get_nft_owner(deps: Deps, nft_id: String, nft_contract_addr: &String) -> StdResult<String> {
   let resp: cw721::OwnerOfResponse = deps
       .querier
       .query_wasm_smart(nft_contract_addr, &nft::msg::QueryMsg::OwnerOf { token_id: nft_id, include_expired: None })?;
   Ok(resp.owner)
}

pub fn get_nft_metadata(deps: Deps, nft_id: String, nft_contract_addr: &String) -> StdResult<Metadata> {
   let resp: cw721::NftInfoResponse<Metadata> = deps
       .querier
       .query_wasm_smart(nft_contract_addr, &nft::msg::QueryMsg::NftInfo { token_id: nft_id })?;
   Ok(resp.extension)
}

pub fn get_staking_bonded (deps: Deps, nft_id: String, staking_contract_addr: &String) -> StdResult<Uint128> {
   let resp: Uint128 = deps
       .querier
       .query_wasm_smart(staking_contract_addr, &staking::msg::QueryMsg::BondedByNFT { nft_id: nft_id })?;
   Ok(resp)
}

// pub struct NftInfoResponse<T> {
//    /// Universal resource identifier for this NFT
//    /// Should point to a JSON file that conforms to the ERC721
//    /// Metadata JSON Schema
//    pub token_uri: Option<String>,
//    /// You can add any custom metadata here when you extend cw721-base
//    pub extension: T,
// }

// #[returns(cw721::NftInfoResponse<Metadata>)]               
// NftInfo {
//     token_id: String,
// },


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