use anchor_lang::prelude::*;
use mpl_core::{ instructions::CreateV2CpiBuilder, ID as MPL_CORE_ID };

use crate::state::Config;

#[derive(Accounts)]
pub struct MintAccessory<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    #[account(mut)]
    pub accessory: Signer<'info>,

    /// CHECK: This is the accessory collection and will be checked by the Metaplex Core program
    #[account(mut)]
    // pub accessory_collection: Option<Account<'info, BaseCollectionV1>>, // This doesn't work. Error: Anchor cannot serialize, deserialize...
    pub accessory_collection: UncheckedAccount<'info>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    /// CHECK: This is the Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> MintAccessory<'info> {
    pub fn mint_accessory(&mut self, name: String, uri: String) -> Result<()> {
        let config_seed_bytes = self.config.seed.to_le_bytes();
        let seeds = &[b"config", config_seed_bytes.as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];

        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.accessory.to_account_info())
            .collection(Some(&self.accessory_collection.to_account_info()))
            .authority(Some(&self.config.to_account_info()))
            .payer(&self.artist.to_account_info())
            .owner(Some(&self.artist.to_account_info()))
            .update_authority(None) // MPL Error: Cannot specify both an update authority and collection on an asset
            .system_program(&self.system_program.to_account_info())
            .name(name)
            .uri(uri)
            .invoke_signed(signer_seeds)?;
        Ok(())
    }
}
