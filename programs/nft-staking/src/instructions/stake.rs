use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;

use crate::constants::*;
use crate::state::ConfigCollection;

#[derive(Accounts)]
pub struct Stake<'info> {
    // The person staking the NFT
    #[account(mut)]
    pub owner: Signer<'info>,
    // You'll need to check if the update authority is correct
    #[account(
        seeds = [AUTHORITY_SEED, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: SystemAccount<'info>,
    // Stake only reads config so it doesn't need to be mutable
    // You are deriving this address so get the bump from config
    #[account(
        seeds = [CONFIG_SEED, collection.key().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, ConfigCollection>,
    /// CHECK: Owned by Metaplex Core; unpacked and verified in stake_asset
    #[account(
        mut,
        owner = MPL_CORE_ID
    )]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: Owned by Metaplex Core; unpacked and verified in stake_asset
    #[account(
        mut,
        owner = MPL_CORE_ID
    )]
    pub collection: UncheckedAccount<'info>,
    /// CHECK: checked by the address constraint to be the Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake_asset(&mut self) -> Result<()> {
        Ok(())
    }
}
