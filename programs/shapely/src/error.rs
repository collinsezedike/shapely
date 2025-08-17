use anchor_lang::prelude::*;

#[error_code]
pub enum ShapelyError {
    #[msg("Invalid Collection Mint")]
    InvalidCollectionMint,

    #[msg("Accessory mint is not verified in the collection")]
    AccessoryNotVerified,

    #[msg("Avatar mint is not verified in the collection")]
    AvatarNotVerified,
}
