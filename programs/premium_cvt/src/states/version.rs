use super::{CollectionAccount, UserStorage};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;
use crate::config::VERSION;


#[account]
pub struct VersionManager {
    pub nonce:u64,
    pub version: String
}

impl VersionManager {
    pub fn is_valid(&self) -> bool{
        return self.version == VERSION
    }
}

#[derive(Accounts)]
pub struct InitVersion<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 24 + 8,
        seeds = [b"Version_Manager"],
        bump
    )]
    pub version_manager: Box<Account<'info, VersionManager>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct CheckVersion<'info> {
    #[account(
        mut,
        seeds = [b"Version_Manager"],
        bump
    )]
    pub version_manager: Box<Account<'info, VersionManager>>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}


#[error_code]
pub enum VersionErrorCode {
    #[msg("only invoke with validator")]
    AuthorityNotValidator,

    #[msg("version not match")]
    VersionNotMatch,

    #[msg("version has been set")]
    VersionHasBeenSet,

}