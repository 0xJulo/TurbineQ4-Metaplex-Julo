use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("You are not authorised to do this")]
    InvalidAuthority,
    #[msg("NFT is not owned by the wallet")]
    InvalidOwner,
    #[msg("NFT is already staked")]
    AlreadyStaked,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("NFT is not currently staked")]
    NotStaked,
    #[msg("Invalid staking timestamp")]
    InvalidTimestamp,
    #[msg("Minimum staking period has not elapsed")]
    FreezePeriodNotElapsed,
}
