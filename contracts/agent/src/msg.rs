use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Uint128, Coin, FullDelegation, Delegation};

#[cw_serde]
pub struct InstantiateMsg {
   pub nft: String,	
   pub staking: String, 
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Bond will bond all staking tokens sent with the message
    Bond {
       nft_id:Option<String>
      
     },
    /// Unbond staking tokens set by amount
    Unbond { 
        nft_id:String
        
    },
    /// Claim is used to claim native tokens previously "unbonded" after the chain-defined unbonding period
    Claim { 
        nft_id:String
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
     GetNFTAdress{},
     #[returns(String)]
     GetStakingAdress{}
}
