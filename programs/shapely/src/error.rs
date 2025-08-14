use anchor_lang::prelude::*;

#[error_code]
pub enum ShapelyError {
    #[msg("Invalid Collection Mint")]
    InvalidCollectionMint,
}
