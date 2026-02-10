use anchor_lang::prelude::*;
use crate::state::Track;

#[derive(Accounts)]
pub struct CreateTrack<'info> {
    #[account(init, payer = artist, space = 8 + 32 + 64)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateTrack>, title: String) -> Result<()> {
    let track = &mut ctx.accounts.track;
    track.artist = *ctx.accounts.artist.key;
    track.title = title;
    Ok(())
}
