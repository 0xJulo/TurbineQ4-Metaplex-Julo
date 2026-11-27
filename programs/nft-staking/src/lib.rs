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

    pub fn create_collection(ctx: Context<CreateCollection>, name: String, uri: String) -> Result<()> {
        ctx.accounts.create_collection(name, uri)
    }
}
