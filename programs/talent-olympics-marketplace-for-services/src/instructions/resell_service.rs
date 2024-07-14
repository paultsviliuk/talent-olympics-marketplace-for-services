use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer };
use crate::ResellService;
use crate::state::Resale;
use crate::errors::ErrorCode;

pub fn resell_service(ctx: Context<ResellService>, resale_price: u64) -> Result<()> {
    let nft = &mut ctx.accounts.nft;
    let buyer = &ctx.accounts.buyer;
    let seller = &ctx.accounts.seller;
    let seller_token_account = &ctx.accounts.seller_token_account;
    let buyer_token_account = &ctx.accounts.buyer_token_account;
    let vendor_token_account = &ctx.accounts.vendor_token_account;

    // Check if the service is not soulbound
    require!(!ctx.accounts.service_listing.is_soulbound, ErrorCode::SoulboundService);

    let royalty_amount = resale_price * 5 / 100;

    // Transfer royalty to original vendor
    let royalty_transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        Transfer {
            from: seller_token_account.to_account_info().clone(),
            to: vendor_token_account.to_account_info().clone(),
            authority: seller.to_account_info().clone(),
        },
    );
    token::transfer(royalty_transfer_ctx, royalty_amount)?;

    // Transfer remaining amount to seller
    let seller_transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        Transfer {
            from: buyer_token_account.to_account_info().clone(),
            to: seller_token_account.to_account_info().clone(),
            authority: buyer.to_account_info().clone(),
        },
    );
    token::transfer(seller_transfer_ctx, resale_price - royalty_amount)?;

    // Update NFT owner and resale history
    nft.current_owner = buyer.key();
    nft.resale_history.push(Resale {
        buyer: buyer.key(),
        price: resale_price,
    });

    Ok(())
}
