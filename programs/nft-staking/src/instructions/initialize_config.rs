use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use mpl_core::accounts::BaseCollectionV1;
use mpl_core::ID as MPL_CORE_ID;

use crate::constants::*;
use crate::error::StakingError;
use crate::state::ConfigCollection;

// To add constraints, ask:
// 1. Is it created or changed?
// 2. Is it a PDA, do we need seeds and a bump?
// 3. Does it need a safety or set up rule?

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Core program checks the collection
    #[account(owner = MPL_CORE_ID)]
    pub collection: UncheckedAccount<'info>,
    // This is like an empty account which the program uses
    // to sign as the update authority, it holds no data.
    // The update authority has already been set when creating a collection
    // So you need to derive it from the seeds it uses.
    #[account(
        seeds = [AUTHORITY_SEED, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: SystemAccount<'info>,
    // This stores the staking rules for the collection, min_days_staked
    // and daily_reward_rate. It also acts as the mint authority
    // for the rewards mint.
    #[account(
        init,
        payer = payer,
        space = ConfigCollection::DISCRIMINATOR.len() + ConfigCollection::INIT_SPACE,
        seeds = [CONFIG_SEED, collection.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, ConfigCollection>,
    // This is the token that acts as rewards for staking
    // It is owned by the config, so only it can create new tokens
    #[account(
        init,
        payer = payer,
        seeds = [REWARDS_SEED, config.key().as_ref()],
        bump,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(
        &mut self,
        min_days_staked: u16,
        daily_reward_rate: u64,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        // Unpack the collection and check it's update authority matches PDA authority
        let base_collection = BaseCollectionV1::try_from(&self.collection.to_account_info())?;
        require!(
            base_collection.update_authority == self.update_authority.key(),
            StakingError::InvalidAuthority
        );
        // save the config by setting inner with the required fields
        self.config.set_inner(ConfigCollection {
            min_days_staked,
            daily_reward_rate,
            config_bump: bumps.config,
            rewards_bump: bumps.rewards_mint,
        });
        Ok(())
    }
}
