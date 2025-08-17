use anchor_lang::{
    prelude::*, solana_program::sysvar::instructions::ID as INSTRUCTIONS_SYSVAR_PROGRAM_ID,
};
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
            types::{Collection, Creator, DataV2},
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{error::ShapelyError, state::Config};

#[derive(Accounts)]
pub struct MintAvatar<'info> {
    #[account(mut)]
    pub collector: Signer<'info>,

    #[account(
        init,
        payer = collector,
        seeds = [b"avatar", collector.key().as_ref(), avatar_collection.key().as_ref()],
        bump,
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
    pub collector_avatar_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            avatar_mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK: This is the avatar mint metadata account
    pub avatar_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            avatar_mint.key().as_ref(),
            b"edition".as_ref(),
            ],
            bump,
            seeds::program = metadata_program.key()
    )]
    /// CHECK: This avatar mint master edition account
    pub avatar_master_edition: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = ["avatar collection".as_bytes(), config.key().as_ref()],
        bump = config.avatar_collection_bump,
        constraint = avatar_collection.key() == config.avatar_collection.key() @ ShapelyError::InvalidCollectionMint
    )]
    pub avatar_collection: Account<'info, Mint>,

    #[account(mut)]
    pub avatar_collection_metadata: Account<'info, MetadataAccount>,

    pub avatar_collection_master_edition: Account<'info, MasterEditionAccount>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(address = INSTRUCTIONS_SYSVAR_PROGRAM_ID)]
    /// CHECK: Sysvar instruction account that is being checked with an address constraint
    pub sysvar_instruction: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub metadata_program: Program<'info, Metadata>,

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

        self.verify_avatar_in_collection(signer_seeds)?;

        Ok(())
    }

    pub fn mint_avatar_nft(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.avatar_mint.to_account_info(),
            to: self.collector_avatar_ata.to_account_info(),
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
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let metadata = &self.avatar_metadata.to_account_info();
        let mint = &self.avatar_mint.to_account_info();
        let authority = &self.config.to_account_info();
        let payer = &self.collector.to_account_info();
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
                    symbol: "SAVATAR".to_owned(),
                    uri,
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: Some(Collection {
                        verified: false,
                        key: self.avatar_collection.key(),
                    }),
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
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

    pub fn verify_avatar_in_collection(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let metadata = &self.avatar_metadata.to_account_info();
        let authority = &self.config.to_account_info();
        let collection_mint = &self.avatar_collection.to_account_info();
        let collection_metadata = &self.avatar_collection_metadata.to_account_info();
        let collection_master_edition = &self.avatar_collection_master_edition.to_account_info();
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
