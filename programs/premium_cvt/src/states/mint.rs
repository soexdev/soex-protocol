use super::CollectionAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(user_storage_index: u32)]
pub struct MintNft<'info> {
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

    #[account(
        mut,
        seeds = [b"user_storage", payer.key().as_ref(), &user_storage_index.to_le_bytes()],
        bump,
    )]
    pub user_storage: Account<'info, UserStorage>,
}

#[derive(Accounts)]
#[instruction(user_storage_index: u32)]
pub struct InitUserStorage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        init,
        payer = payer,
        space = 8 + 516,
        seeds = [b"user_storage", payer.key().as_ref(), &user_storage_index.to_le_bytes()],
        bump,
    )]
    pub user_storage: Account<'info, UserStorage>,
}

#[account]
pub struct UserStorage {
    pub position: u32,
    pub minted:[UserStorageMinted; 5]
}

impl UserStorage {
    pub fn is_nft_owner(&self, nft_id: u32) -> bool {
        for i in 0..self.position as usize {
            let start = self.minted[i].current;
            let end = start + self.minted[i].copies;
            if nft_id > start && nft_id <= end{
                return true;
            }
        }
        false
    }
}
#[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct UserStorageMinted {
    pub current: u32,
    pub copies: u32,
}

#[event]
pub struct MintNftEvent {
    pub who: Pubkey,
    pub current: u32,
    pub copies: u32,
    pub user_storage_index: u32,
    pub phase: u32,
    pub quit_deadline: u64,
}