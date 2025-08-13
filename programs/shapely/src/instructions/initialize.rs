use anchor_lang::prelude::*;
use mpl_core::{
    accounts::BaseCollectionV1, instructions::CreateCollectionV1CpiBuilder, ID as MPL_CORE_ID,
};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub avatar_collection: Signer<'info>,

    #[account(mut)]
    pub accessory_collection: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::SPACE
    )]
    pub config: Account<'info, Config>,

    #[account(seeds = [b"treasury", config.key().as_ref()], bump)]
    pub treasury: SystemAccount<'info>,

    /// CHECK: This is the Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64, fee: u16, bumps: &InitializeBumps) -> Result<()> {

        self.initialize_config(seed, fee, bumps)?;

        self.mint_avatar_collection()?;

        self.mint_accessory_collection()?;

        Ok(())
    }

    pub fn initialize_config(
        &mut self,
        seed: u64,
        fee: u16,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            bump: bumps.config,
            treasury_bump: bumps.treasury,
            seed,
            fee,
        });

        Ok(())
    }

    pub fn mint_avatar_collection(&mut self) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.avatar_collection.to_account_info())
            .update_authority(Some(&self.config.to_account_info()))
            .payer(&self.payer.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name("Shapely Avatar Collection".to_string())
            .uri("https://github.com/collinsezedike/shapely".to_string())
            .invoke()?;

        Ok(())
    }

    pub fn mint_accessory_collection(&mut self) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.accessory_collection.to_account_info())
            .update_authority(Some(&self.config.to_account_info()))
            .payer(&self.payer.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name("Shapely Accessory Collection".to_string())
            .uri("https://github.com/collinsezedike/shapely".to_string())
            .invoke()?;

        Ok(())
    }
}
