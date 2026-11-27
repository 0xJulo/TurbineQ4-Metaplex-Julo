use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ConfigCollection {
    // Admin to set the min days a NFT must be staked
    pub min_days_staked: u16,
    // How much rewards a user will get per day
    pub daily_reward_rate: u64,
    pub config_bump: u8,
    pub rewards_bump: u8,
}
