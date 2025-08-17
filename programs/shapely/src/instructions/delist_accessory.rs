use anchor_lang::prelude::*;
use anchor_spl::token::{
    close_account, transfer_checked, CloseAccount, Mint, Token, TokenAccount, TransferChecked,
};

use crate::state::{Config, Listing};

#[derive(Accounts)]
pub struct DelistAccessory<'info> {
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

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

impl<'info> DelistAccessory<'info> {
    pub fn delist_accessory(&mut self) -> Result<()> {
        let seeds = &[
            b"listing",
            self.accessory_mint.to_account_info().key.as_ref(),
            self.artist.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        self.withdraw_nft(signer_seeds)?;
        self.close_vault(signer_seeds)?;
        Ok(())
    }

    pub fn withdraw_nft(&mut self, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.listing_vault.to_account_info(),
            mint: self.accessory_mint.to_account_info(),
            to: self.artist_accessory_ata.to_account_info(),
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
