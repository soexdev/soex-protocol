use crate::errors::ErrorCode;
use crate::states::Transfer;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

pub fn handle_verify_cvt_sol_account(ctx: Context<Transfer>) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(collection.admin == ctx.accounts.payer.key(), ErrorCode::InvalidAdmin);
    require!(*ctx.accounts.cvt_sol_account.key != Pubkey::default(), ErrorCode::ZeroAddressError);
    require!(*ctx.accounts.cvt_sol_account.key == collection.cvt_sol_account, ErrorCode::InvalidCvtSOLAccount);

    let input_amount = 10_000_000; // 0.01 * 1_000_000_000
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.payer.to_account_info().clone(),
            to: ctx.accounts.cvt_sol_account.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, input_amount)?;

    collection.authority |= 0x01<<2;
    msg!("Verify successfully");

    Ok(())
}
