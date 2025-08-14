// use anchor_lang::prelude::*;
// use mpl_core::{ instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID };

// use crate::state::Config;

// #[derive(Accounts)]
// pub struct ListAccessory<'info> {
//     #[account(mut)]
//     pub artist: Signer<'info>,

//     #[account(mut)]
//     pub accessory: Signer<'info>,

//     /// CHECK: This is the accessory collection and will be checked by the Metaplex Core program
//     #[account(mut)]
//     pub accessory_collection: UncheckedAccount<'info>,

//     #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
//     pub config: Account<'info, Config>,

//     /// CHECK: This is the Metaplex Core program
//     #[account(address = MPL_CORE_ID)]
//     pub mpl_core_program: UncheckedAccount<'info>,

//     pub system_program: Program<'info, System>,
// }

// impl<'info> ListAccessory<'info> {
//     pub fn list_accessory(&mut self, name: String, uri: String) -> Result<()> {
//         Ok(())
//     }
// }
