use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::types::{Attributes, Attribute, Plugin, PluginType, PluginAuthority, UpdateAuthority};
use mpl_core::fetch_plugin;
use mpl_core::instructions::{AddPluginV1CpiBuilder, UpdatePluginV1CpiBuilder};

use crate::constants::*;
use crate::state::ConfigCollection;
use crate::error::StakingError;

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
    pub fn stake_asset(&mut self, bumps: &StakeBumps) -> Result<()> {
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
        // We need to remember the timestamp of when an NFT was staked
        let current_time = Clock::get()?.unix_timestamp;

        // Check to see if the NFT already has an Attributes plugin on it
        match fetch_plugin::<BaseAssetV1, Attributes>(
            &self.asset.to_account_info(),
            PluginType::Attributes,
        ) {
            Err(_) => {
                // There is no attributes, so create two attributes for the NFT
                // as staked being true and the current timestamp.
                AddPluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
                    .asset(&self.asset.to_account_info())
                    .collection(Some(&self.collection.to_account_info()))
                    .payer(&self.owner.to_account_info())
                    .authority(Some(&self.update_authority.to_account_info()))
                    .system_program(&self.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes {
                        attribute_list: vec![
                            Attribute {
                                key: "staked".to_string(),
                                value: "true".to_string(),
                            },
                            Attribute {
                                key: "staked_at".to_string(),
                                value: current_time.to_string(),
                            },
                        ],
                    }))
                    .init_authority(PluginAuthority::UpdateAuthority)
                    .invoke_signed(signer_seeds)?;
            }
            Ok((_, attrs, _)) => {
                // The NFT does have some attributes, we need to make sure
                // it is not already staked, and keep the other attributes
                let mut attribute_list: Vec<Attribute> = Vec::new();
                for attribute in attrs.attribute_list {
                    if attribute.key == "staked" {
                        require!(attribute.value == "false", StakingError::AlreadyStaked);
                    } else if attribute.key != "staked_at" {
                        attribute_list.push(attribute);
                    }
                }
                attribute_list.push(Attribute {
                    key: "staked".to_string(),
                    value: "true".to_string(),
                });
                attribute_list.push(Attribute {
                    key: "staked_at".to_string(),
                    value: current_time.to_string(),
                });
                // Send the new attributes to the NFT onchain
                UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
                    .asset(&self.asset.to_account_info())
                    .collection(Some(&self.collection.to_account_info()))
                    .payer(&self.owner.to_account_info())
                    .authority(Some(&self.update_authority.to_account_info()))
                    .system_program(&self.system_program.to_account_info())
                    .plugin(Plugin::Attributes(Attributes { attribute_list }
                    ))
                    .invoke_signed(signer_seeds)?;
            }
        }

        Ok(())
    }
}
