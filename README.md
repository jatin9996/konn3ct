# konn3ctKonn3ct Smart Contract
Overview
The konn3ct smart contract is designed for a decentralized NFT community platform on the Solana blockchain. It enables users to stake NFTs, claim rewards based on staking periods, and create community events. This contract leverages the Anchor framework for Solana development, providing a robust and secure environment for decentralized applications.
Features
NFT Staking: Users can stake their NFTs to participate in the community and earn rewards over time.
Reward Claims: Stakers can claim their accumulated rewards after a minimum staking period.
Event Creation: Community organizers can create events, setting details like title, description, start time, and end time.
Prerequisites
Install Rust: https://www.rust-lang.org/tools/install
Install Solana CLI: https://docs.solana.com/cli/install-solana-cli-tools
Install Anchor: https://project-serum.github.io/anchor/getting-started/installation.html
Setup
1. Clone the repository:
2. Build the smart contract:
3. Deploy the smart contract to Solana devnet (ensure you have set up your Solana CLI for devnet):
UsageUsage
Staking an NFT
To stake an NFT, call the stake_nft function with the NFT's mint address and the staker's account information.
Claiming Rewards
After the minimum staking period, rewards can be claimed through the claim_rewards function by providing the staker's and the staked NFT's account information.
Creating an Event
Community organizers can create events by calling the create_event function with the event details.
Development
For local development and testing:
1. Start a local Solana test validator:
