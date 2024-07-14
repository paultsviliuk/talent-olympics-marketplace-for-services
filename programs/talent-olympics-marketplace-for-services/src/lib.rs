use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;

declare_id!("2gWedLpUCkJUUyLyfjJnAAaxNBRvaxzBBJTSoq87MTTr");

#[program]
pub mod talent_olympics_marketplace_for_services {
    use super::*;

    pub fn register_vendor(ctx: Context<RegisterVendor>, name: String) -> Result<()> {
        instructions::register_vendor(ctx, name)
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
