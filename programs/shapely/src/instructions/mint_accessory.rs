use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::{
            instructions::{
                CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
                CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
                CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs,
                VerifyCollectionV1Cpi, VerifyCollectionV1CpiAccounts,
            },
            types::{CollectionDetails, Creator, DataV2},
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{error::ShapelyError, state::Config};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintAccessory<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    #[account(
        init,
        payer = artist,
        mint::decimals = 0,
        mint::authority = config,
        mint::freeze_authority = config
    )]
    pub accessory_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = artist,
        associated_token::mint = accessory_mint,
        associated_token::authority = artist
    )]
    pub artist_accessory_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            accessory_mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK: This is the accessory mint metadata account and will be initialized by the metaplex program
    pub accessory_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            accessory_mint.key().as_ref(),
            b"edition".as_ref(),
            ],
            bump,
            seeds::program = metadata_program.key()
    )]
    /// CHECK: This accessory mint master edition account and will be initialized by the metaplex program
    pub accessory_master_edition: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = ["accessory collection".as_bytes(), config.key().as_ref()],
        bump = config.accessory_collection_bump,
        constraint = accessory_collection.key() == config.accessory_collection.key() @ ShapelyError::InvalidCollectionMint
    )]
    pub accessory_collection: Account<'info, Mint>,

    #[account(mut)]
    pub accessory_collection_metadata: Account<'info, MetadataAccount>,

    pub accessory_collection_master_edition: Account<'info, MasterEditionAccount>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    /// CHECK: Sysvar instruction account that is being checked with an address constraint
    pub sysvar_instruction: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl<'info> MintAccessory<'info> {
    pub fn mint_accessory(&mut self, name: String, uri: String) -> Result<()> {
        let config_seed_bytes = self.config.seed.to_le_bytes();
        let seeds = &[b"config", config_seed_bytes.as_ref(), &[self.config.bump]];
        let signer_seeds = &[&seeds[..]];

        self.mint_accessory_nft(signer_seeds)?;

        self.create_accessory_metadata(name, uri, signer_seeds)?;

        self.create_accessory_master_edition(signer_seeds)?;

        self.verify_accessory_in_collection(signer_seeds)?;

        Ok(())
    }

    pub fn mint_accessory_nft(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.accessory_mint.to_account_info(),
            to: self.artist_accessory_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn create_accessory_metadata(
        &mut self,
        name: String,
        uri: String,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let metadata = &self.accessory_metadata.to_account_info();
        let mint = &self.accessory_mint.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.artist.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

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
                    name,
                    symbol: "SACCESSORY".to_owned(),
                    uri,
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

    pub fn create_accessory_master_edition(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let master_edition = &self.accessory_master_edition.to_account_info();
        let metadata = &self.accessory_metadata.to_account_info();
        let mint = &self.accessory_mint.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.artist.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();
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
            },
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn verify_accessory_in_collection(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let metadata = &self.accessory_metadata.to_account_info();
        let authority = &self.config.to_account_info();
        let collection_mint = &self.accessory_collection.to_account_info();
        let collection_metadata = &self.accessory_collection_metadata.to_account_info();
        let collection_master_edition = &self.accessory_collection_master_edition.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instruction.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        let verify_collection = VerifyCollectionV1Cpi::new(
            metadata_program,
            VerifyCollectionV1CpiAccounts {
                authority,
                delegate_record: None,
                metadata,
                collection_mint,
                collection_metadata: Some(collection_metadata),
                collection_master_edition: Some(collection_master_edition),
                system_program,
                sysvar_instructions,
            },
        );

        verify_collection.invoke_signed(signer_seeds)?;

        msg!("Collection Verified!");

        Ok(())
    }
}
