use anchor_lang::prelude::*;

#[account]
pub struct Vendor {
    pub owner: Pubkey,
    pub name: String,
    pub services: Vec<Pubkey>,
}
