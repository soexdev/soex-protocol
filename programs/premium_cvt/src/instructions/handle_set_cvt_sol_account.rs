use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::config::{VALIDATOR_PUBKEY, VERSION};
use crate::errors::ErrorCode;
use crate::{utils, ID};
use crate::states::{CollectionAccount, VersionManager};
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;

#[derive(Accounts)]
pub struct SetCvtSolAccount<'info> {
    #[account(
        mut,
        seeds = [
            "collection".as_bytes()
        ],
        bump
    )]
    pub global_config: AccountLoader<'info, CollectionAccount>,

    #[account(
        mut,
        seeds = [b"Version_Manager"],
        bump
    )]
    pub version_manager: Box<Account<'info, VersionManager>>,

    #[account(mut,address = global_config.load().unwrap().admin)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,
}

pub fn handle_set_cvt_sol_account(ctx: Context<SetCvtSolAccount>, cvt_sol_account: Pubkey, nonce:u64, signature: [u8; 64]) -> Result<()> {
    if ctx.accounts.signer.is_signer == false {
        return Err(ErrorCode::InvalidSigner.into());
    }

    let global_config = &mut ctx.accounts.global_config.load_mut()?;
    let address_nonce = ctx.accounts.version_manager.nonce + 1;
    assert_eq!(address_nonce, nonce);

    let mut msg = vec![];
    msg.extend(VERSION.as_bytes());
    msg.extend(nonce.to_le_bytes());
    msg.extend(global_config.cvt_sol_account.to_bytes());
    msg.extend(cvt_sol_account.to_bytes());
    msg.extend("set_cvt_sol_account".as_bytes());
    msg.extend(ID.to_bytes());

    let hash = keccak::hash(&msg).to_bytes();
    msg!("hash {}", Pubkey::new_from_array(hash));

    let validator_pubkey = Pubkey::from_str(VALIDATOR_PUBKEY).unwrap();
    let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.ix_sysvar)?;
    utils::verify_ed25519_ix(&ix, &validator_pubkey.to_bytes(), &hash, &signature)?;

    global_config.cvt_sol_account = cvt_sol_account;
    global_config.authority &= !(0x01 << 2);

    ctx.accounts.version_manager.nonce = nonce;
    msg!("set_cvt_sol_account {} {}", global_config.authority, cvt_sol_account);

    Ok(())
}
