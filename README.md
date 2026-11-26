# Assignment: NFT Staking with Metaplex Core

The goal of this assignment is to:

1. Write an NFT staking program for Metaplex Core assets, where a staked NFT is frozen and earns reward tokens over time.
2. Add a `claim_rewards` instruction that mints the accumulated rewards to the user's Associated Token Account, keeping the NFT staked and frozen.
3. Add a `burn_staked_nft` instruction that uses the BurnDelegate plugin to permanently burn a staked NFT for a large one-time reward bonus.
4. Track a `total_staked` counter as an Attribute on the Collection account, going up on stake and down on unstake.

And optionally:

5. Add an Oracle plugin to the Collection so NFTs can only be transferred during set hours (for example 9AM to 5PM UTC).
6. Write a permissionless instruction that reads the on-chain time and updates the Oracle's Transfer validation state, rewarding the caller for running it close to the open or close time.
7. Write an instruction that transfers the NFT through the program.

The program is written from scratch, using the tutor's reference Core staking program as a guide.

## Code Documentation

- Anchor: https://www.anchor-lang.com/docs
- Metaplex Core: https://www.metaplex.com/docs/smart-contracts/core
- Core plugins: https://www.metaplex.com/docs/smart-contracts/core/plugins
- Oracle plugin: https://www.metaplex.com/docs/smart-contracts/core/external-plugins/oracle
- Program Derived Addresses: https://solana.com/docs/core/pda
- Cross Program Invocation: https://solana.com/docs/core/cpi
- Tokens on Solana: https://solana.com/docs/tokens

## Results
