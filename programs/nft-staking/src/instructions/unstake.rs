use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    mint_to_checked,
    Mint,
    MintToChecked,
    TokenAccount,
    TokenInterface,
};
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::types::{
    Attributes,
    Attribute,
    FreezeDelegate,
    Plugin,
    PluginType,
    UpdateAuthority,
};
use mpl_core::fetch_plugin;
use mpl_core::instructions::{
    UpdateCollectionPluginV1CpiBuilder,
    UpdatePluginV1CpiBuilder,
};

use crate::constants::*;
use crate::state::ConfigCollection;
use crate::error::StakingError;

#[derive(Accounts)]
pub struct Unstake<'info> {
    // The person unstaking the NFT
    #[account(mut)]
    pub owner: Signer<'info>,
    // You'll need to check if the update authority is correct
    #[account(
        seeds = [AUTHORITY_SEED, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: SystemAccount<'info>,
    // Unstake only reads config so it doesn't need to be mutable
    // You are deriving this address so get the bump from config
    #[account(
        seeds = [CONFIG_SEED, collection.key().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, ConfigCollection>,
    // The rewards token that you created.
    #[account(
        mut,
        seeds = [REWARDS_SEED, config.key().as_ref()],
        bump = config.rewards_bump,
        mint::token_program = token_program,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    // User's token wallet for receiving reward tokens (created if they don't have one yet)
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = rewards_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_rewards_ata: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: Owned by Metaplex Core; unpacked and verified in unstake_asset
    #[account(
        mut,
        owner = MPL_CORE_ID
    )]
    pub asset: UncheckedAccount<'info>,
    /// CHECK: Owned by Metaplex Core; unpacked and verified in unstake_asset
    #[account(
        mut,
        owner = MPL_CORE_ID
    )]
    pub collection: UncheckedAccount<'info>,
    /// CHECK: checked by the address constraint to be the Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake_asset(&mut self, bumps: &UnstakeBumps) -> Result<()> {
        // Asset and collection are UncheckedAccounts, so we need to
        // check that Metaplex Core owns them
        let base_asset = BaseAssetV1::try_from(&self.asset.to_account_info())?;
        // Make sure the caller owns the NFT
        require!(base_asset.owner == self.owner.key(), StakingError::InvalidOwner);
        // Make sure the NFT belongs to the collection
        require!(
            base_asset.update_authority == UpdateAuthority::Collection(self.collection.key()),
            StakingError::InvalidAuthority
        );
        // Unpack the collection and make sure our update authority controls the PDA
        let base_collection = BaseCollectionV1::try_from(&self.collection.to_account_info())?;
        require!(
            base_collection.update_authority == self.update_authority.key(),
            StakingError::InvalidAuthority
        );

        // Build the signer seeds for the update authority
        let seed_bytes = self.collection.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            AUTHORITY_SEED,
            seed_bytes.as_ref(),
            &[bumps.update_authority],
        ]];
        // We need the current timestamp to calculate how long the NFT was staked
        let current_time = Clock::get()?.unix_timestamp;

        // Read the NFT's Attributes plugin — fail if it was never staked
        let (_, attrs, _) = fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(),
            PluginType::Attributes,
        )
        .map_err(|_| StakingError::NotStaked)?;

        // Check staked == "true", read staked_at, and reset both while keeping other attributes
        let mut attribute_list: Vec<Attribute> = Vec::new();
        let mut is_staked = false;
        let mut staked_at_value: Option<String> = None;

        for attribute in attrs.attribute_list {
            match attribute.key.as_str() {
                "staked" => {
                    require!(attribute.value == "true", StakingError::NotStaked);
                    is_staked = true;
                }
                "staked_at" => {
                    staked_at_value = Some(attribute.value);
                }
                _ => {
                    attribute_list.push(attribute);
                }
            }
        }

        require!(is_staked, StakingError::NotStaked);

        let staked_at_timestamp = staked_at_value
            .ok_or(StakingError::InvalidTimestamp)?
            .parse::<i64>()
            .map_err(|_| StakingError::InvalidTimestamp)?;

        // Check that enough full days have passed since staking
        let elapsed_seconds = current_time
            .checked_sub(staked_at_timestamp)
            .ok_or(StakingError::InvalidTimestamp)?;
        let staked_days = elapsed_seconds
            .checked_div(SECONDS_PER_DAY)
            .ok_or(StakingError::InvalidTimestamp)?;

        require!(
            staked_days >= self.config.min_days_staked as i64,
            StakingError::FreezePeriodNotElapsed
        );

        // Reset staking attributes on the NFT
        attribute_list.push(Attribute {
            key: "staked".to_string(),
            value: "false".to_string(),
        });
        attribute_list.push(Attribute {
            key: "staked_at".to_string(),
            value: "0".to_string(),
        });

        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.owner.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::Attributes(Attributes { attribute_list }))
            .invoke_signed(signer_seeds)?;

        // Unfreeze the NFT so the owner can transfer it again
        UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.owner.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .invoke_signed(signer_seeds)?;

        // Decrement total_staked on the Collection's Attributes plugin
        let (_, collection_attrs, _) = fetch_plugin::<BaseCollectionV1, Attributes>(
            &self.collection.to_account_info(),
            PluginType::Attributes,
        )
        .map_err(|_| StakingError::NotStaked)?;

        let mut collection_attribute_list: Vec<Attribute> = Vec::new();
        let mut total_staked: u64 = 0;

        for attribute in collection_attrs.attribute_list {
            if attribute.key == "total_staked" {
                total_staked = attribute
                    .value
                    .parse::<u64>()
                    .map_err(|_| StakingError::Overflow)?;
            } else {
                collection_attribute_list.push(attribute);
            }
        }

        let new_total = total_staked
            .checked_sub(1)
            .ok_or(StakingError::Overflow)?;

        collection_attribute_list.push(Attribute {
            key: "total_staked".to_string(),
            value: new_total.to_string(),
        });

        UpdateCollectionPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.owner.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::Attributes(Attributes {
                attribute_list: collection_attribute_list,
            }))
            .invoke_signed(signer_seeds)?;

        // Calculate reward tokens: staked_days * daily_reward_rate
        let staked_days_u64 = u64::try_from(staked_days)
            .map_err(|_| StakingError::InvalidTimestamp)?;
        let reward_amount = staked_days_u64
            .checked_mul(self.config.daily_reward_rate)
            .ok_or(StakingError::Overflow)?;

        // Mint reward tokens to the user's ATA, signed by the config PDA (mint authority)
        if reward_amount > 0 {
            let config_signer_seeds: &[&[&[u8]]] = &[&[
                CONFIG_SEED,
                seed_bytes.as_ref(),
                &[self.config.config_bump],
            ]];

            let cpi_ctx = CpiContext::new_with_signer(
                self.token_program.key(),
                MintToChecked {
                    mint: self.rewards_mint.to_account_info(),
                    to: self.owner_rewards_ata.to_account_info(),
                    authority: self.config.to_account_info(),
                },
                config_signer_seeds,
            );
            mint_to_checked(cpi_ctx, reward_amount, self.rewards_mint.decimals)?;
        }

        Ok(())
    }
}
