use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub bump: u8,
    pub treasury_bump: u8,
    pub avatar_collection_bump: u8,
    pub accessory_collection_bump: u8,
    pub avatar_collection: Pubkey,
    pub accessory_collection: Pubkey,
    /// Accessory sales commission (in basis point e.g 1000 = 10%)
    pub fee: u16,
    /// Config ID
    pub seed: u64,
}

impl Config {
    pub const SPACE: usize = 8 + Config::INIT_SPACE;
}
