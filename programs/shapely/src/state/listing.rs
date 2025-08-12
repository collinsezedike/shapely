use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub bump: u8,
    pub units: u16,
    pub price: u64,
    pub accessory_mint: Pubkey,
}

impl Listing {
    pub const SPACE: usize = 8 + Listing::INIT_SPACE;
}
