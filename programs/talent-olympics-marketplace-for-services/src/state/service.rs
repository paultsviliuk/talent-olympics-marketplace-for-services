use anchor_lang::prelude::*;

/// Represents vendors services and created by vendors. 
#[account]
pub struct ServiceListing {
    pub vendor: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub is_soulbound: bool,
    pub metadata_uri: String,
}

/// ServiceNFT is minted when a service is purchased by consumer.
#[account]
pub struct ServiceNFT {
    pub original_service: Pubkey,
    pub current_owner: Pubkey,
    pub metadata_pda: Pubkey,
    pub resale_history: Vec<Resale>,
}

#[account]
pub struct Metadata {
    pub metadata_uri: String,
}


#[account]
pub struct Resale {
    pub buyer: Pubkey,
    pub price: u64,
}
