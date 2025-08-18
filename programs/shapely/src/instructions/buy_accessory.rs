use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{
        close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked,
    },
};

use crate::{
    error::ShapelyError,
    state::{Config, Listing},
};

#[derive(Accounts)]
pub struct BuyAccessory<'info> {
    #[account(mut)]
    pub collector: Signer<'info>,

    #[account(
        seeds = [b"avatar", collector.key().as_ref(), config.avatar_collection.key().as_ref()],
        bump,
    )]
    pub collector_avatar_mint: Account<'info, Mint>,

    #[account(
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            collector_avatar_mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key(),

        constraint = collector_avatar_metadata.collection.as_ref().unwrap().verified == true @ ShapelyError::AvatarNotVerified,
        constraint = collector_avatar_metadata.collection.as_ref().unwrap().key.as_ref() ==
        config.avatar_collection.key().as_ref() @ ShapelyError::AvatarNotVerified
    )]
    pub collector_avatar_metadata: Account<'info, MetadataAccount>,

    #[account(mut)]
    /// CHECK: This will be validated in the listing seeds
    pub artist: AccountInfo<'info>,

    pub accessory_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = collector,
        associated_token::mint = accessory_mint,
        associated_token::authority = collector
    )]
    pub collector_accessory_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = artist,
        seeds = [b"listing", accessory_mint.key().as_ref(), artist.key().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = accessory_mint,
        associated_token::authority = listing
    )]
    pub listing_vault: Account<'info, TokenAccount>,

    #[account(seeds = [b"config", config.seed.to_le_bytes().as_ref()], bump = config.bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [b"treasury", config.key().as_ref()], bump = config.treasury_bump)]
    pub treasury: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl<'info> BuyAccessory<'info> {
    pub fn buy_accessory(&mut self) -> Result<()> {
        let seeds = &[
            b"listing",
            self.accessory_mint.to_account_info().key.as_ref(),
            self.artist.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        self.process_payment()?;
        self.withdraw_nft(signer_seeds)?;
        self.close_vault(signer_seeds)?;
        Ok(())
    }

    pub fn process_payment(&mut self) -> Result<()> {
        let listing_fee = ((self.config.fee / 10_000) as u64) * self.listing.price;
        let amount_to_pay_artist = self.listing.price - listing_fee;

        // 1. Collect fees
        let cpi_accounts = Transfer {
            from: self.collector.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, listing_fee)?;

        // 2. Payout artist
        let cpi_accounts = Transfer {
            from: self.collector.to_account_info(),
            to: self.artist.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount_to_pay_artist)?;

        Ok(())
    }

    pub fn withdraw_nft(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.listing_vault.to_account_info(),
            mint: self.accessory_mint.to_account_info(),
            to: self.collector_accessory_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(
            cpi_ctx,
            self.listing_vault.amount,
            self.accessory_mint.decimals,
        )?;

        Ok(())
    }

    pub fn close_vault(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.listing_vault.to_account_info(),
            destination: self.artist.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
