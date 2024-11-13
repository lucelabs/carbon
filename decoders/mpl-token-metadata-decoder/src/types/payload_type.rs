
use super::*;
use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum PayloadType {
    Pubkey
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
    Seeds
                (
                    SeedsVec,
                )
    ,
    MerkleProof
                (
                    LeafInfo,
                )
    ,
    Number
                (
                    u64,
                )
    ,
}



