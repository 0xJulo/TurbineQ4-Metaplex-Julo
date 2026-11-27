use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
use mpl_core::ID as MPL_CORE_ID;

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
    pub update_authority: SystemAccount<'info>,
    pub config: Account<'info, ConfigCollection>,
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfig<'info> {
    pub fn initialize_config(&mut self) -> Result<()> {
        Ok(())
    }
}
