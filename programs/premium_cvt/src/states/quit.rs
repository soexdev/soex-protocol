use super::{CollectionAccount, UserStorage};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;
use crate::config::VERSION;

#[derive(Accounts)]
#[instruction(user_storage_index: u32)]
pub struct QuitNft<'info> {
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

    #[account(
        mut,
        seeds = [b"ADDRESS", payer.key().as_ref()],
        bump
    )]
    pub address_manager: Box<Account<'info, AccountManager>>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [b"sol_account"],
        bump,
    )]
    pub sol_account: AccountInfo<'info>,

    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"user_storage", payer.key().as_ref(), &user_storage_index.to_le_bytes()],
        bump,
    )]
    pub user_storage: Account<'info, UserStorage>,
}

#[event]
pub struct QuitNftEvent {
    pub who: Pubkey,
    pub nft_id: u16,
}

#[account]
pub struct AccountManager {
    pub nonce: u64,
}


#[derive(Accounts)]
pub struct InitAddressManager<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8+8,
        seeds = [b"ADDRESS", payer.key().as_ref()],
        bump
    )]
    pub address_manager: Box<Account<'info, AccountManager>>,
    pub system_program: Program<'info, System>,
}