use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("You are not authorised to do this")]
    InvalidAuthority,
    #[msg("NFT is not owned by the wallet")]
    InvalidOwner,
    #[msg("NFT is already staked")]
    AlreadyStaked,
}
