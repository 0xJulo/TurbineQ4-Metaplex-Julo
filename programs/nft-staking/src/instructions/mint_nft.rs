use anchor_lang::prelude::*;
use mpl_core::ID as MPL_CORE_ID;
use mpl_core::instructions::CreateV2CpiBuilder;

use crate::constants::*;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Update authority is already created when the collection is created
    // When minting we need to check for this PDA. 
    #[account(
        seeds = [AUTHORITY_SEED, collection.key().as_ref()],
        bump,
    )]
    pub update_authority: SystemAccount<'info>,
    /// CHECK: Core program check the collection
    #[account(mut, owner = MPL_CORE_ID)]
    pub collection: UncheckedAccount<'info>,
    // Create the address where the NFT will be minted
    #[account(mut)]
    pub collection_asset: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: checked by the address constraint to be the Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self,
        name: String,
        uri: String,
        bumps: &MintNFTBumps,
    ) -> Result<()> {
        // Build the signer seeds for the update authority PDA
        // so the program can sign on its behalf
        let seed_bytes = self.collection.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
           AUTHORITY_SEED,
            seed_bytes.as_ref(),
            &[bumps.update_authority],
        ]];
        // Call Metaplex Core to create the NFT, it has to be
        // inside the collection, owned by the payer, and approved by the update authority.
        CreateV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.payer.to_account_info())
            .authority(Some(&self.update_authority.to_account_info()))
            .name(name)
            .uri(uri)
            .asset(&self.collection_asset.to_account_info())
            .owner(Some(&self.payer.to_account_info()))
            .system_program(&self.system_program.to_account_info()
        ).invoke_signed(signer_seeds)?;
        Ok(())
    }
}