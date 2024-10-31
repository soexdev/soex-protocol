use super::CollectionAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            "collection".as_bytes()
        ],
        bump,
    )]
    pub collection: AccountLoader<'info, CollectionAccount>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"sol_account"],
        bump,
    )]
    pub sol_account: AccountInfo<'info>,

    #[account(mut)]
    pub cvt_sol_account: AccountInfo<'info>,
}
