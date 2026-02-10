use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Reward {}

pub fn handler(_ctx: Context<Reward>, _amount: u64) -> Result<()> {
    Ok(())
}
