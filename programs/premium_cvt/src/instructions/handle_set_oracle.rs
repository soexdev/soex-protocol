use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::keccak;
use anchor_lang::solana_program::sysvar::instructions::load_instruction_at_checked;
use crate::errors::ErrorCode;
use crate::states::{AccountManager, CollectionAccount};
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;
use crate::config::{VALIDATOR_PUBKEY, VERSION};
use crate::{ID, utils};

#[derive(Accounts)]
pub struct SetOracle<'info> {
    #[account(
        mut,
        seeds = [
            "collection".as_bytes()
        ],
        bump
    )]
    pub global_config: AccountLoader<'info, CollectionAccount>,


    #[account(mut)]
    pub validator: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"ADDRESS", validator.key().as_ref()],
        bump
    )]
    pub validator_manager: Box<Account<'info, AccountManager>>,

    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    #[account(mut,address = global_config.load().unwrap().admin)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct NewOracleEvent {
    old_oracle: Pubkey,
    new_oracle: Pubkey,
}

pub fn handle_set_oracle(ctx: Context<SetOracle>, oracle: Pubkey, nonce:u64, signature: [u8; 64]) -> Result<()> {
    if ctx.accounts.signer.is_signer == false {
        return Err(ErrorCode::InvalidSigner.into());
    }

    let global_config = &mut ctx.accounts.global_config.load_mut()?;
    let old_oracle =global_config.oracle;

    let addr_nonce = ctx.accounts.validator_manager.nonce + 1;
    assert_eq!(addr_nonce, nonce);
    assert_ne!(global_config.oracle,global_config.oracle2);


    let mut msg = vec![];
    msg.extend(old_oracle.to_bytes());
    msg.extend(oracle.to_bytes());
    msg.extend(nonce.to_le_bytes());
    msg.extend("set_oracle".as_bytes());
    msg.extend(ID.to_bytes());
    msg.extend(VERSION.as_bytes());

    let hash = keccak::hash(&msg).to_bytes();
    msg!("hash {}", Pubkey::new_from_array(hash));

    let validator_pubkey = Pubkey::from_str(VALIDATOR_PUBKEY).unwrap();
    let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.ix_sysvar)?;
    utils::verify_ed25519_ix(&ix, &validator_pubkey.to_bytes(), &hash, &signature)?;

    global_config.oracle = oracle;

    emit!(NewOracleEvent {
        old_oracle,
        new_oracle: oracle
    });
    ctx.accounts.validator_manager.nonce = nonce;

    Ok(())
}

pub fn handle_set_oracle2(ctx: Context<SetOracle>, oracle: Pubkey, nonce:u64, signature: [u8; 64]) -> Result<()> {
    if ctx.accounts.signer.is_signer == false {
        return Err(ErrorCode::InvalidSigner.into());
    }

    let global_config = &mut ctx.accounts.global_config.load_mut()?;
    let old_oracle =global_config.oracle2;

    let addr_nonce = ctx.accounts.validator_manager.nonce + 1;
    assert_eq!(addr_nonce, nonce);
    assert_ne!(global_config.oracle,global_config.oracle2);

    let mut msg = vec![];
    msg.extend(old_oracle.to_bytes());
    msg.extend(oracle.to_bytes());
    msg.extend(nonce.to_le_bytes());
    msg.extend("set_oracle2".as_bytes());
    msg.extend(ID.to_bytes());
    msg.extend(VERSION.as_bytes());

    let hash = keccak::hash(&msg).to_bytes();
    msg!("hash {}", Pubkey::new_from_array(hash));

    let validator_pubkey = Pubkey::from_str(VALIDATOR_PUBKEY).unwrap();
    let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.ix_sysvar)?;
    utils::verify_ed25519_ix(&ix, &validator_pubkey.to_bytes(), &hash, &signature)?;

    global_config.oracle2 = oracle;

    emit!(NewOracleEvent {
        old_oracle,
        new_oracle: oracle
    });

    ctx.accounts.validator_manager.nonce = nonce;

    Ok(())
}
