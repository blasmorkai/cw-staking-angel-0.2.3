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
}
