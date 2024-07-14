use anchor_lang::prelude::*;
use crate::RegisterVendor;

pub fn register_vendor(ctx: Context<RegisterVendor>, name: String) -> Result<()> {
    let vendor = &mut ctx.accounts.vendor;
    vendor.owner = ctx.accounts.user.key();
    vendor.name = name;
    vendor.services = Vec::new();
    Ok(())
}
