use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::instructions::CreateCollectionV2CpiBuilder;

use crate::constants::*;

// This struct is about creating a collection
// The config side of things is connected to this collection.
// The config is made later, in its own instruction
// and is linked to the collection by its seeds.

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    // The person who pays for the creation of the collection
    #[account(mut)]
    pub payer: Signer<'info>,
    // This is the role to update the collection
    // adding things like plugins and freezing the collection
    #[account(
        seeds = [AUTHORITY_SEED, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: SystemAccount<'info>,
    // The collection is something that is made by Metaplex
    // This is a new keypair whose address becomes the collection address
    #[account(mut)]
    pub collection: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: checked by the address constraint to be the Metaplex Core program
    // UncheckedAccount means Anchor does no checks, the address constraint
    // above this is the check.
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> CreateCollection<'info> {
    // Metaplex Core requires the name and uri to be able to create a collection.
    pub fn create_collection(
        &mut self,
        name: String,
        uri: String,
    ) -> Result<()> {
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.payer.to_account_info())
            .update_authority(Some(&self.update_authority.to_account_info()))
            .name(name)
            .uri(uri)
            .system_program(&self.system_program.to_account_info()
        ).invoke()?;
        Ok(())
    }
}