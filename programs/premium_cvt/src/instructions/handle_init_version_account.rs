use crate::states::*;
use anchor_lang::prelude::*;
use crate::config::VERSION;

pub fn handle_init_version_account(_ctx: Context<InitVersion>) ->Result<()> {
    msg!("version account has been init to {}",_ctx.accounts.version_manager.key());

    Ok(())
}