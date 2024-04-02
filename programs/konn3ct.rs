use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("FILL_IN_YOUR_PROGRAM_ID_HERE");

#[program]
pub mod nft_community_platform {
    use super::*;

    pub fn stake_nft(ctx: Context<StakeNft>, nft_mint: Pubkey) -> Result<()> {
        // Transfer the NFT from the staker's account to the staking pool account
        let cpi_accounts = Transfer {
            from: ctx.accounts.staker_nft_account.to_account_info(),
            to: ctx.accounts.staking_pool.to_account_info(),
            authority: ctx.accounts.staker_info.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        // Update the StakedNft account
        let staked_nft = &mut ctx.accounts.staked_nft;
        staked_nft.staker = *ctx.accounts.staker_info.key;
        staked_nft.nft_mint = nft_mint;
        staked_nft.stake_time = Clock::get()?.unix_timestamp;

        // Emit an event
        emit!(NftStaked {
            staker: *ctx.accounts.staker_info.key,
            nft_mint: nft_mint,
            stake_time: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        // 1. Verify the claimant's eligibility
        // Ensure the NFT is staked
        if !ctx.accounts.staked_nft.is_staked {
            return Err(ErrorCode::NftNotStaked.into());
        }

        // Check if the staking period has been met
        let now = Clock::get()?.unix_timestamp;
        if now - ctx.accounts.staked_nft.stake_time < MIN_STAKING_PERIOD {
            return Err(ErrorCode::StakingPeriodNotMet.into());
        }

        // Verify the claimant is the owner of the staked NFT or authorized
        if ctx.accounts.claimant.key() != ctx.accounts.staked_nft.staker {
            return Err(ErrorCode::Unauthorized.into());
        }

        // 2. Calculate the reward
        let reward_amount = calculate_reward(&ctx.accounts.staked_nft);

        // Proceed with transferring the reward
        let cpi_accounts = Transfer {
            from: ctx.accounts.rewards_pool.to_account_info(),
            to: ctx.accounts.claimant_reward_account.to_account_info(),
            authority: ctx.accounts.rewards_pool.to_account_info(), // Assuming the rewards pool is the authority or has delegated authority
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, reward_amount)?;

        Ok(())
    }

    pub fn create_event(ctx: Context<CreateEvent>, event_details: EventDetails) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.title = event_details.title;
        event.description = event_details.description;
        event.start_time = event_details.start_time;
        event.end_time = event_details.end_time;
        event.organizer = *ctx.accounts.authority.key;
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(init, payer = staker_info, space = 8 + 32 + 32 + 8)]
    pub staked_nft: Account<'info, StakedNft>,
    // Other accounts...
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub rewards_pool: Account<'info, TokenAccount>, // The account holding the rewards tokens
    #[account(mut)]
    pub claimant_reward_account: Account<'info, TokenAccount>, // The claimant's account to receive the rewards
    pub token_program: Program<'info, Token>, // The SPL Token program
    // Other accounts...
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = authority, space = 8 + 256 + 256 + 8 + 8 + 32)]
    pub event: Account<'info, EventDetails>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    // Additional accounts for event management
}

#[account]
pub struct StakedNft {
    pub staker: Pubkey,
    pub nft_mint: Pubkey,
    pub stake_time: i64, // Unix timestamp
    pub is_staked: bool,
}

#[account]
pub struct EventDetails {
    pub title: String,
    pub description: String,
    pub start_time: i64, // Unix timestamp
    pub end_time: i64, // Unix timestamp
    pub organizer: Pubkey,
}

#[event]
pub struct NftStaked {
    pub staker: Pubkey,
    pub nft_mint: Pubkey,
    pub stake_time: i64,
}

// Helper function to calculate rewards (simplified example)
fn calculate_reward(staked_nft: &Account<StakedNft>) -> u64 {
    // Implement your reward calculation logic here
    let now = Clock::get().unwrap().unix_timestamp;
    let staking_duration = now - staked_nft.stake_time; // in seconds
    staking_duration as u64 / 86400 // Example: 1 token per day of staking
}

// Helper function to transfer rewards (simplified example)
fn transfer_rewards(
    from_pool: &AccountInfo, 
    to_claimant: &AccountInfo, 
    authority: &AccountInfo, 
    token_program: &AccountInfo, 
    amount: u64
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from_pool.clone(),
        to: to_claimant.clone(),
        authority: authority.clone(),
    };
    let cpi_program = token_program.clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

// Additional structs and enums as needed
#[error_code]
pub enum ErrorCode {
    #[msg("The NFT is not staked.")]
    NftNotStaked,
    #[msg("The staking period has not been met.")]
    StakingPeriodNotMet,
    #[msg("Unauthorized claim attempt.")]
    Unauthorized,
}
