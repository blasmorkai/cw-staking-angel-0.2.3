use cosmwasm_std::{StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unable to instantiate NFT contract")]
    NFTContractNotInstantiated {},

    #[error("Reply not handled. reply_id: {id}")]
    UnknownReplyIdSubMsgResult { id: String },
    
    #[error("Unable to instantiate Staking contract")]
    StakingContractNotInstantiated {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("No funds sent")]
    NoFunds {},

    #[error("Multiple denoms sent")]
    MultipleDenoms {},

    #[error("Invalid Coin BBB")]
    InvalidCoin {},

    #[error("Not the owner of the NFT")]
    NotOwnerNFT {},

    #[error("Only a Bonded NFT can be rebonded")]
    OnlyBondedNFTCanBeRebonded {},

    #[error("Only a Bonded NFT can be unbonded")]
    OnlyBondedNFTCanBeUnbonded {},

    #[error("Only an Unbonding NFT can be claimed")]
    OnlyUnbondingNFTCanBeClaimed {},

    #[error("NFT only supports one native coin")]
    OnlyOneNativeCoinPerNFT {},

    #[error("Unable to mint NFT")]
    UnableMintNFT {},

    #[error("Unable to update NFT Metadata")]
    UnableUpdateNFTMetadata {},
    
    #[error("Unable to stake based on a newly minted NFT")]
    UnableToStakeBondNewNFT {},
    
    #[error("Unable to stake based on a rebonded/updated NFT")]
    UnableToStakeReBondNFT {},
    
    #[error("Unable to unbond staking")]
    UnableToUnbondStaking {},

    #[error("Unable to unbond NFT")]
    UnableToUnbondNFT {},

    #[error("Unable to burn NFT")]
    UnableToBurnNFT {},

    #[error("Unable to claim staking")]
    UnableToClaimStaking {},

    #[error("NFT amount mismatch nft: {nft}  staking: {staking}")]
    NFTStakingMismatch { staking: String, nft:String }
}
