use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ mint_to, Mint, MintTo, Token, TokenAccount },
    metadata::{
        Metadata,
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi,
                CreateMasterEditionV3CpiAccounts,
                CreateMasterEditionV3InstructionArgs,
                CreateMetadataAccountV3Cpi,
                CreateMetadataAccountV3CpiAccounts,
                CreateMetadataAccountV3InstructionArgs,
            },
            types::{ CollectionDetails, Creator, DataV2 },
        },
    },
};

use crate::{ error::ShapelyError, state::Config };

#[derive(Accounts)]
pub struct MintAvatar<'info> {
    #[account(mut)]
    pub collector: Signer<'info>,

    #[account(
        init,
        payer = collector,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config
    )]
    pub avatar_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = collector,
        associated_token::mint = avatar_mint,
        associated_token::authority = collector
    )]
    pub collector_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = avatar_collection.key() == config.avatar_collection.key() @ ShapelyError::InvalidCollectionMint
    )]
    pub avatar_collection: Account<'info, Mint>,

    /// CHECK: This is the avatar metadata account and will be initialized by the metaplex program
    #[account(mut)]
    pub avatar_metadata: UncheckedAccount<'info>,

    /// CHECK: This avatar master edition account and will be initialized by the metaplex program
    #[account(mut)]
    pub avatar_master_edition: UncheckedAccount<'info>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl<'info> MintAvatar<'info> {
    pub fn mint_avatar(&mut self, name: String, uri: String) -> Result<()> {
        let config_seed_bytes = self.config.seed.to_le_bytes();
        let seeds = &[b"config", config_seed_bytes.as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];

        self.mint_avatar_nft(signer_seeds)?;

        self.create_avatar_metadata(name, uri, signer_seeds)?;

        self.create_avatar_master_edition(signer_seeds)?;

        Ok(())
    }

    pub fn mint_avatar_nft(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.avatar_mint.to_account_info(),
            to: self.collector_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn create_avatar_metadata(
        &mut self,
        name: String,
        uri: String,
        signer_seeds: &[&[&[u8]]]
    ) -> Result<()> {
        let metadata = &self.avatar_metadata.to_account_info();
        let mint = &self.avatar_mint.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.collector.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();

        let creator = vec![Creator {
            address: self.collector.key().clone(),
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
                    name,
                    symbol: "SAVATAR".to_owned(),
                    uri,
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None,
                },
                is_mutable: true,
                collection_details: Some(CollectionDetails::V1 {
                    size: 0,
                }),
            }
        );
        metadata_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn create_avatar_master_edition(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let master_edition = &self.avatar_master_edition.to_account_info();
        let metadata = &self.avatar_metadata.to_account_info();
        let mint = &self.avatar_mint.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.collector.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.token_metadata_program.to_account_info();
        let token_program = &self.token_program.to_account_info();

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: master_edition,
                update_authority: authority,
                mint_authority: authority,
                mint,
                payer,
                metadata,
                token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            }
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        Ok(())
    }
}
