use std::str::FromStr;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::config::{VALIDATOR_PUBKEY, VERSION};
use crate::{ID, utils};

pub fn handle_check_version(ctx: Context<CheckVersion>, code:String, nonce:u64, signature: [u8; 64]) ->Result<()> {

    require_eq!(VALIDATOR_PUBKEY,ctx.accounts.payer.key().to_string(),VersionErrorCode::AuthorityNotValidator);

    let address_nonce = ctx.accounts.version_manager.nonce + 1;
    assert_eq!(address_nonce, nonce);

    // msg [VERSION.as_bytes()|nonce.to_le_bytes()|code.as_bytes()|ID.to_bytes()]
    let mut msg = vec![];
    msg.extend(VERSION.as_bytes());
    msg.extend(nonce.to_le_bytes());
    msg.extend(code.as_bytes());
    msg.extend(ID.to_bytes());

    let hash = keccak::hash(&msg).to_bytes();
    msg!("hash {}", Pubkey::new_from_array(hash));

    let validator_pubkey = Pubkey::from_str(VALIDATOR_PUBKEY).unwrap();
    let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.ix_sysvar)?;
    utils::verify_ed25519_ix(&ix, &validator_pubkey.to_bytes(), &hash, &signature)?;

    ctx.accounts.version_manager.nonce = nonce;
    ctx.accounts.version_manager.version = VERSION.to_string();
    msg!("version verify  ok");

    Ok(())
}