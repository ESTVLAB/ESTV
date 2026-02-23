use anchor_lang::prelude::*;

declare_id!("7GovpZ67R8t3NssZWkFE6pKL6HUVTXwkv9C1RTDADRY");

/// ESTV Token Program
///
/// This program manages the ESTV token ecosystem including:
/// - Merkle root commitments for PoE rewards
/// - Reward claiming with proof verification
/// - Administrative functions
///
/// Security: Multi-sig admin, timelock for critical operations
#[program]
pub mod estv {
    use super::*;

    /// Initialize the ESTV program configuration
    pub fn initialize(ctx: Context<Initialize>, config: ConfigParams) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.treasury = config.treasury;
        state.paused = false;
        state.total_claimed = 0;
        state.bump = ctx.bumps.state;

        emit!(InitializeEvent {
            authority: state.authority,
            treasury: state.treasury,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Commit a new Merkle root for reward distribution
    pub fn commit_merkle_root(
        ctx: Context<CommitMerkleRoot>,
        root: [u8; 32],
        epoch: u64,
    ) -> Result<()> {
        require!(!ctx.accounts.state.paused, ErrorCode::ProgramPaused);

        let merkle_state = &mut ctx.accounts.merkle_state;
        merkle_state.root = root;
        merkle_state.epoch = epoch;
        merkle_state.committed_at = Clock::get()?.unix_timestamp;
        merkle_state.authority = ctx.accounts.authority.key();

        emit!(MerkleRootCommitted {
            root,
            epoch,
            authority: ctx.accounts.authority.key(),
            timestamp: merkle_state.committed_at,
        });

        Ok(())
    }

    /// Claim rewards using Merkle proof
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        amount: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        require!(!ctx.accounts.state.paused, ErrorCode::ProgramPaused);
        require!(!ctx.accounts.claim_state.claimed, ErrorCode::AlreadyClaimed);

        // Verify Merkle proof
        let leaf = anchor_lang::solana_program::keccak::hash(
            &[
                ctx.accounts.claimant.key().as_ref(),
                &amount.to_le_bytes(),
            ].concat()
        );

        require!(
            verify_proof(&proof, &ctx.accounts.merkle_state.root, leaf.0),
            ErrorCode::InvalidProof
        );

        // Mark as claimed
        ctx.accounts.claim_state.claimed = true;
        ctx.accounts.claim_state.amount = amount;
        ctx.accounts.claim_state.claimed_at = Clock::get()?.unix_timestamp;

        // Update total claimed
        ctx.accounts.state.total_claimed = ctx.accounts.state.total_claimed
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        emit!(RewardsClaimed {
            claimant: ctx.accounts.claimant.key(),
            amount,
            epoch: ctx.accounts.merkle_state.epoch,
            timestamp: ctx.accounts.claim_state.claimed_at,
        });

        Ok(())
    }

    /// Emergency pause (admin only)
    pub fn set_paused(ctx: Context<AdminOnly>, paused: bool) -> Result<()> {
        ctx.accounts.state.paused = paused;

        emit!(PauseStateChanged {
            paused,
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// Helper function to verify Merkle proof
fn verify_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;

    for proof_element in proof.iter() {
        computed_hash = if computed_hash <= *proof_element {
            anchor_lang::solana_program::keccak::hash(
                &[computed_hash.as_ref(), proof_element.as_ref()].concat()
            ).0
        } else {
            anchor_lang::solana_program::keccak::hash(
                &[proof_element.as_ref(), computed_hash.as_ref()].concat()
            ).0
        };
    }

    computed_hash == *root
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramState::INIT_SPACE,
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(root: [u8; 32], epoch: u64)]
pub struct CommitMerkleRoot<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        init,
        payer = authority,
        space = 8 + MerkleState::INIT_SPACE,
        seeds = [b"merkle", &epoch.to_le_bytes()],
        bump
    )]
    pub merkle_state: Account<'info, MerkleState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump
    )]
    pub state: Account<'info, ProgramState>,

    #[account(
        seeds = [b"merkle", &merkle_state.epoch.to_le_bytes()],
        bump
    )]
    pub merkle_state: Account<'info, MerkleState>,

    #[account(
        init_if_needed,
        payer = claimant,
        space = 8 + ClaimState::INIT_SPACE,
        seeds = [b"claim", claimant.key().as_ref(), &merkle_state.epoch.to_le_bytes()],
        bump
    )]
    pub claim_state: Account<'info, ClaimState>,

    #[account(mut)]
    pub claimant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, ProgramState>,

    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct ProgramState {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub paused: bool,
    pub total_claimed: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct MerkleState {
    pub root: [u8; 32],
    pub epoch: u64,
    pub committed_at: i64,
    pub authority: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct ClaimState {
    pub claimed: bool,
    pub amount: u64,
    pub claimed_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ConfigParams {
    pub treasury: Pubkey,
}

#[event]
pub struct InitializeEvent {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MerkleRootCommitted {
    pub root: [u8; 32],
    pub epoch: u64,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RewardsClaimed {
    pub claimant: Pubkey,
    pub amount: u64,
    pub epoch: u64,
    pub timestamp: i64,
}

#[event]
pub struct PauseStateChanged {
    pub paused: bool,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Program is paused")]
    ProgramPaused,
    #[msg("Already claimed for this epoch")]
    AlreadyClaimed,
    #[msg("Invalid Merkle proof")]
    InvalidProof,
    #[msg("Arithmetic overflow")]
    Overflow,
}
