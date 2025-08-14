pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9PgEiZqE6d9CxAUY7gF9Tn2mXeySnJPnUkMhRAnxwskX");

#[program]
pub mod shapely {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts.initialize(seed, fee, &ctx.bumps)
    }

    // pub fn mint_accessory(ctx: Context<MintAccessory>, name: String, uri: String) -> Result<()> {
    //     ctx.accounts.mint_accessory(name, uri)
    // }

    pub fn mint_avatar(ctx: Context<MintAvatar>, name: String, uri: String) -> Result<()> {
        ctx.accounts.mint_avatar(name, uri)
    }
}
