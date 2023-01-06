use cosmwasm_std::{StdError, Uint128, Uint64};
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

    #[error("Invalid Coin")]
    InvalidCoin {},

    #[error("Not the owner of the NFT")]
    NotOwnerNFT {},

    #[error("Can not rebond and unbonding NFT")]
    UnbondingNFT {},

    #[error("NFT only supports one native coin")]
    OnlyOneNativeCoinPerNFT {},

    #[error("Unable to create NFT")]
    UnableCreateNFT {},

    #[error("Unable to updae NFT Metadata")]
    UnableUpdateNFTMetadata {},
    
    #[error("NFT amount mismatch nft: {nft}  staking: {staking}")]
    NFTStakingMismatch { staking: String, nft:String }
}
