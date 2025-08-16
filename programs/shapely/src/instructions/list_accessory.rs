use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{
    error::ShapelyError,
    state::{Config, Listing},
};

#[derive(Accounts)]
pub struct ListAccessory<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,

    pub accessory_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = accessory_mint,
        associated_token::authority = artist
    )]
    pub artist_accessory_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = ["accessory collection".as_bytes(), config.key().as_ref()],
        bump = config.accessory_collection_bump,
        constraint = accessory_collection.key() == config.accessory_collection.key() @ ShapelyError::InvalidCollectionMint
    )]
    pub accessory_collection: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            accessory_mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key(),

        // constraint = accessory_metadata.collection.as_ref().unwrap().verified == true,
        // constraint = accessory_metadata.collection.as_ref().unwrap().key.as_ref() ==
        // accessory_collection.key().as_ref()
    )]
    pub accessory_metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            accessory_mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub accessory_master_edition: Account<'info, MasterEditionAccount>,

    #[account(
        init,
        payer = artist,
        seeds = [b"listing", accessory_mint.key().as_ref(), artist.key().as_ref()],
        bump,
        space = Listing::SPACE
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init,
        payer = artist,
        associated_token::mint = accessory_mint,
        associated_token::authority = listing
    )]
    pub listing_vault: Account<'info, TokenAccount>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl<'info> ListAccessory<'info> {
    pub fn list_accessory(&mut self, price: u64, bumps: &ListAccessoryBumps) -> Result<()> {
        self.initialize_listing(price, bumps.listing)?;
        self.deposit_nft()?;
        Ok(())
    }

    pub fn initialize_listing(&mut self, price: u64, bump: u8) -> Result<()> {
        self.listing.set_inner(Listing {
            bump,
            price,
            accessory_mint: self.accessory_mint.key(),
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.artist_accessory_ata.to_account_info(),
            mint: self.accessory_mint.to_account_info(),
            to: self.listing_vault.to_account_info(),
            authority: self.artist.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(
            cpi_ctx,
            self.artist_accessory_ata.amount,
            self.accessory_mint.decimals,
        )?;

        Ok(())
    }
}
