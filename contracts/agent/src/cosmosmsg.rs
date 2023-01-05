
use cosmwasm_std::{
   coin, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env,
   MessageInfo, QuerierWrapper, Response, StakingMsg, StdResult, Uint128, Uint64,
   Order, Coin, DistributionMsg, CosmosMsg, SubMsg, entry_point, WasmMsg, Reply,
   SubMsgResult
};
use cw721_base::MintMsg;
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
    token_id: &Addr,
    token_uri: Option<String>,
    extension: Metadata,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    // create mint msg
    let mint_msg = nft::msg::ExecuteMsg::Mint(MintMsg { token_id: token_id.into(), owner:owner.into(), token_uri, extension });
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
    token_id: &Addr,
    token_uri: Option<String>,
    extension: Metadata,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    // create transfer cw20 msg
    let update_msg = nft::msg::ExecuteMsg::UpdateMetadata { token_id: token_id.into(), token_uri, extension }; 
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
    token_id: &Addr,
    nft_contract_address: &Addr
 ) -> StdResult<CosmosMsg> {
    // create transfer cw20 msg
    let burn_msg = nft::msg::ExecuteMsg::Burn { token_id:token_id.into() }; 
    let exec_burn = WasmMsg::Execute {
        contract_addr: nft_contract_address.into(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    };
    let burn_cosmos_msg: CosmosMsg = exec_burn.into();
    Ok(burn_cosmos_msg)
 }