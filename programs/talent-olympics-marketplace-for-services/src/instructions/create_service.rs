use anchor_lang::prelude::*;
use crate::ListService;

/// Create new service by vendor.
pub fn create_service(
    ctx: Context<ListService>,
    name: String,
    description: String,
    price: u64,
    is_soulbound: bool,
    metadata_uri: String,
) -> Result<()> {
    let service = &mut ctx.accounts.service;
    service.vendor = ctx.accounts.vendor.key();
    service.name = name;
    service.description = description;
    service.price = price;
    service.is_soulbound = is_soulbound;
    service.metadata_uri = metadata_uri;

    let vendor = &mut ctx.accounts.vendor;
    vendor.services.push(service.key());
    Ok(())
}
