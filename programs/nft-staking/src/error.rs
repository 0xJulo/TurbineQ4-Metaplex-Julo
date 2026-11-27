use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("You are not authorised to do this")]
    InvalidAuthority,
}
