use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub bump: u8,
    pub units: u16,
    pub price: u64,
    pub accessory_mint: Pubkey,
}
