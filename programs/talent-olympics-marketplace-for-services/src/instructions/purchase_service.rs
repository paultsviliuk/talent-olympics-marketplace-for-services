use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use crate::PurchaseService;

pub fn purchase_service(ctx: Context<PurchaseService>) -> Result<()> {
    let service_listing: &Account<_> = &ctx.accounts.service_listing;
    let service_nft: &mut Account<_> = &mut ctx.accounts.service_nft;
    let buyer = &ctx.accounts.buyer;
    let mint: &Account<_> = &ctx.accounts.mint;
    let metadata_pda = &ctx.accounts.metadata_pda;
    let token_program: &Program<_> = &ctx.accounts.token_program;
    
    // Transfer payment from buyer to vendor
    anchor_lang::solana_program::program::invoke(
        &anchor_lang::solana_program::system_instruction::transfer(
            &buyer.key(),
            &service_listing.vendor,
            service_listing.price,
        ),
        &[
            buyer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Mint the ServiceNFT
    let cpi_accounts = MintTo {
        mint: mint.to_account_info(),
        to: ctx.accounts.service_nft_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;

    // Initialize the ServiceNFT
    service_nft.original_service = service_listing.key();
    service_nft.current_owner = buyer.key();
    service_nft.metadata_pda = metadata_pda.key();
    service_nft.resale_history = vec![];

    // Initialize metadata in PDA
    let metadata_account = &mut ctx.accounts.metadata_account;
    metadata_account.metadata_uri = service_listing.metadata_uri.clone();

    Ok(())
}
