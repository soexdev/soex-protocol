use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

pub fn handle_set_authority(ctx: Context<Config>, authority: u32) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(
            collection.admin == ctx.accounts.payer.key(),
            ErrorCode::InvalidAdmin
        );
    require!(
            collection.authority != authority,
            ErrorCode::AuthorityAlreadySet
        );

    collection.authority = authority;
    msg!("set_authority {}", authority);
    emit!(AuthorityEvent {authority});

    Ok(())
}