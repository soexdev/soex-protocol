use anchor_lang::prelude::*;
use crate::config::VERSION;

#[derive(Accounts)]
pub struct GetVersion<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_get_version(ctx: Context<GetVersion>) -> Result<String> {
    Ok(VERSION.to_string())
}