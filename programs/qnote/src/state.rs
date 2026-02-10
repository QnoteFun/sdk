use anchor_lang::prelude::*;

#[account]
pub struct Track {
    pub artist: Pubkey,
    pub title: String,
}
