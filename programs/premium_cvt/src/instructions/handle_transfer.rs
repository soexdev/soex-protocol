use crate::errors::ErrorCode;
use crate::states::Transfer;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

pub fn handle_transfer(ctx: Context<Transfer>) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(((collection.authority >> 1) & 0x01) == 1, ErrorCode::InvalidAuthority);
    require!(((collection.authority >> 2) & 0x01) == 1, ErrorCode::CvtSOLAccountNotVerify);
    require!(((collection.authority >> 3) & 0x01) == 0, ErrorCode::AlreadyTransfer);
    require!(collection.admin == ctx.accounts.payer.key(), ErrorCode::InvalidAdmin);
    require!(*ctx.accounts.cvt_sol_account.key == collection.cvt_sol_account, ErrorCode::InvalidCvtSOLAccount);
    let input_amount = collection.price * collection.lock_mint_amount as u64;
    let seeds: &[&[u8]] = &[b"sol_account", &[ctx.bumps.sol_account]];
    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.sol_account.to_account_info(),
                to: ctx.accounts.cvt_sol_account.to_account_info(),
            },
            &[seeds],
        ),
        input_amount,
    )?;

    collection.authority |= 0x01<<3;
    msg!("Transfer successfully");

    Ok(())
}
