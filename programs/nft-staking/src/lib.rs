pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5QSBQgDmDTapeQgjU1V7ppZCmhFQnyUBPNwQ6XGMumr3");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn create_collection(
        ctx: Context<CreateCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create_collection(name, uri)
    }

    pub fn mint_nft(ctx: Context<MintNFT>, name: String, uri: String) -> Result<()> {
        ctx.accounts.mint_nft(name, uri, &ctx.bumps)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        min_days_staked: u16,
        daily_reward_rate: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(min_days_staked, daily_reward_rate, &ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake_asset(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake_asset(&ctx.bumps)
    }
}
