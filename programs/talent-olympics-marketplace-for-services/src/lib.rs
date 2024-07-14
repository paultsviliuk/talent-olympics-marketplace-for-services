use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
pub mod state;
pub mod instructions;
pub mod errors;
pub mod constants;

declare_id!("9SysbfwgBy6RAp75myGwuhLCfBWnohoJvvaKXV9fgE5C");

#[program]
pub mod talent_olympics_marketplace_for_services {
    use super::*;

    pub fn register_vendor(ctx: Context<RegisterVendor>, name: String) -> Result<()> {
        instructions::register_vendor(ctx, name)
    }

    pub fn create_service(
        ctx: Context<ListService>,
        name: String,
        description: String,
        price: u64,
        is_soulbound: bool,
        metadata_uri: String,
    ) -> Result<()> {
        instructions::create_service(ctx, name, description, price, is_soulbound, metadata_uri)
    }

    pub fn resell_service(ctx: Context<ResellService>, price: u64) -> Result<()> {
        instructions::resell_service(ctx, price)
    }
}

#[derive(Accounts)]
pub struct RegisterVendor<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 256 + 4 + (32 * 100))]
    pub vendor: Account<'info, state::Vendor>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListService<'info> {
    #[account(init, payer = vendor, space = 8 + 32 + 32 + 64 + 8 + 8 + 32)]
    pub service: Account<'info, state::ServiceListing>,
    #[account(mut, signer)]
    pub vendor: Account<'info, state::Vendor>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseService<'info> {
    #[account(mut)]
    pub service_listing: Account<'info, state::ServiceListing>,
    #[account(init, payer = buyer, space = 8 + 32 + 32 + 32 + (32 + 8) * 10)]
    pub service_nft: Account<'info, state::ServiceNFT>,
    #[account(init, payer = buyer, space = 8 + 256, seeds = [b"metadata", service_nft.key().as_ref()], bump)]
    pub metadata_account: Account<'info, state::Metadata>,
    pub metadata_pda: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub service_nft_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResellService<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vendor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub nft: Account<'info, state::ServiceNFT>,
    #[account(mut)]
    pub service_listing: Account<'info, state::ServiceListing>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
