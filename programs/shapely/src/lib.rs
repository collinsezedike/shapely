pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

// declare_id!("9PgEiZqE6d9CxAUY7gF9Tn2mXeySnJPnUkMhRAnxwskX");     // For localnet
declare_id!("3ccHCxQuyua3zePL3t9Nu7p4CR27CaKasSctiA8Zh1sb");    // For devnet

#[program]
pub mod shapely {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize(seed, fee, &ctx.bumps)
    }

    pub fn mint_accessory(ctx: Context<MintAccessory>, name: String, uri: String) -> Result<()> {
        ctx.accounts.mint_accessory(name, uri)
    }

    pub fn mint_avatar(ctx: Context<MintAvatar>, name: String, uri: String) -> Result<()> {
        ctx.accounts.mint_avatar(name, uri)
    }

    pub fn list_accessory(ctx: Context<ListAccessory>, price: u64) -> Result<()> {
        ctx.accounts.list_accessory(price, &ctx.bumps)
    }

    pub fn delist_accessory(ctx: Context<DelistAccessory>) -> Result<()> {
        ctx.accounts.delist_accessory()
    }

    pub fn buy_accessory(ctx: Context<BuyAccessory>) -> Result<()> {
        ctx.accounts.buy_accessory()
    }
}
