use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("QNoTe11111111111111111111111111111111111");

#[program]
pub mod qnote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn create_track(ctx: Context<CreateTrack>, title: String) -> Result<()> {
        create_track::handler(ctx, title)
    }

    pub fn reward(ctx: Context<Reward>, amount: u64) -> Result<()> {
        reward::handler(ctx, amount)
    }
}
