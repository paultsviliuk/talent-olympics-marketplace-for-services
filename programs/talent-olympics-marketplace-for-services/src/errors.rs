use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("This service is already owned by the buyer.")]
    AlreadyOwned,
    #[msg("You aren't the owner of this service.")]
    NotOwner,
    #[msg("This service is soulbound and can't be resold.")]
    SoulboundService,
}
