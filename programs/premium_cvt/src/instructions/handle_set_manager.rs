use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::CollectionAccount;

#[derive(Accounts)]
pub struct SetManager<'info> {
    #[account(
        mut,
        seeds = [
            "collection".as_bytes()
        ],
        bump
    )]
    pub global_config: AccountLoader<'info, CollectionAccount>,

    #[account(mut,address = global_config.load().unwrap().admin)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct NewManagerEvent {
    old_manager: Pubkey,
    new_manager: Pubkey,
}

pub fn handle_set_manager(ctx: Context<SetManager>, manager: Pubkey) -> Result<()> {

    require!(Pubkey::default() != manager,ErrorCode::ZeroAddressError);

    if ctx.accounts.signer.is_signer == false {
        return Err(ErrorCode::InvalidSigner.into());
    }

    let global_config = &mut ctx.accounts.global_config.load_mut()?;
    global_config.admin = manager;

    emit!(NewManagerEvent {
        old_manager: ctx.accounts.signer.key(),
        new_manager: manager
    });

    Ok(())
}
