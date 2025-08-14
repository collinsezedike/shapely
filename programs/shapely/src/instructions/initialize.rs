use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
                CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
                CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs,
            },
            types::{CollectionDetails, Creator, DataV2},
        },
        Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config
    )]
    pub avatar_collection: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config
    )]
    pub accessory_collection: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = avatar_collection,
        associated_token::authority = config
    )]
    pub avatar_collection_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = accessory_collection,
        associated_token::authority = config
    )]
    pub accessory_collection_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub avatar_collection_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub accessory_collection_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub avatar_collection_master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub accessory_collection_master_edition: UncheckedAccount<'info>,

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

    associated_token_program: Program<'info, AssociatedToken>,

    token_program: Program<'info, Token>,

    token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, seed: u64, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        let config_seed_bytes = seed.to_le_bytes();
        let seeds = &[b"config", config_seed_bytes.as_ref(), &[bumps.config]];
        let signer_seeds = &[&seeds[..]];

        self.initialize_config(seed, fee, bumps)?;

        self.mint_avatar_collection(signer_seeds)?;

        self.mint_accessory_collection(signer_seeds)?;

        self.create_avatar_collection_metadata(signer_seeds)?;

        self.create_accessory_collection_metadata(signer_seeds)?;

        self.create_avatar_collection_master_edition(signer_seeds)?;

        self.create_accessory_collection_master_edition(signer_seeds)?;

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
            avatar_collection: self.avatar_collection.key(),
            accessory_collection: self.accessory_collection.key(),
            seed,
            fee,
        });

        Ok(())
    }

    pub fn mint_avatar_collection(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.avatar_collection.to_account_info(),
            to: self.avatar_collection_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn mint_accessory_collection(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.accessory_collection.to_account_info(),
            to: self.accessory_collection_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn create_avatar_collection_metadata(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let metadata = &self.avatar_collection_metadata.to_account_info();
        let mint = &self.avatar_collection.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.payer.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();

        let creator = vec![Creator {
            address: self.config.key().clone(),
            verified: true,
            share: 100,
        }];

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Shapely Avatar Collection".to_owned(),
                    symbol: "SAVACOL".to_owned(),
                    uri: "https://github.com/collinsezedike/shapely".to_owned(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: Some(CollectionDetails::V1 { size: 0 }),
            },
        );
        metadata_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn create_accessory_collection_metadata(
        &mut self,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let metadata = &self.accessory_collection_metadata.to_account_info();
        let mint = &self.accessory_collection.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.payer.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();

        let creator = vec![Creator {
            address: self.config.key().clone(),
            verified: true,
            share: 100,
        }];

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Shapely Accessory Collection".to_owned(),
                    symbol: "SACCCOL".to_owned(),
                    uri: "https://github.com/collinsezedike/shapely".to_owned(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: Some(CollectionDetails::V1 { size: 0 }),
            },
        );
        metadata_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn create_avatar_collection_master_edition(
        &mut self,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let edition = &self.avatar_collection_master_edition.to_account_info();
        let mint = &self.avatar_collection.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.payer.to_account_info();
        let metadata = &self.avatar_collection_metadata.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition,
                mint,
                update_authority: authority,
                mint_authority: authority,
                payer,
                metadata,
                token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn create_accessory_collection_master_edition(
        &mut self,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let edition = &self.accessory_collection_master_edition.to_account_info();
        let mint = &self.accessory_collection.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.payer.to_account_info();
        let metadata = &self.accessory_collection_metadata.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition,
                mint,
                update_authority: authority,
                mint_authority: authority,
                payer,
                metadata,
                token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        Ok(())
    }
}
