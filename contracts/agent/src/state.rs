use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Uint128, Uint64};
use cw_storage_plus::{Item, MultiIndex, Index, IndexList, IndexedMap, Map};
use cw_utils::Duration;



// Addresses for contracts
pub const STAKING: Item<String> = Item::new("staking");
pub const NFT: Item<String> = Item::new("nft");

