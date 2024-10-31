use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

pub fn handle_set_phase_supply_quit_deadline(ctx: Context<SetPhaseSupply>, phase:u32, quit_deadline:u64) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(collection.admin == ctx.accounts.payer.key(),ErrorCode::InvalidAdmin);
    require!(phase < collection.phase_supply_quit_deadline.len() as u32, ErrorCode::InvalidPhase);

    let timestamp = Clock::get()?.unix_timestamp as u64;
    assert!(quit_deadline>=timestamp);

    collection.phase_supply_quit_deadline[phase as usize] = quit_deadline;

    msg!("set_phase_supply_quit_deadline {} {}", phase, quit_deadline);
    emit!(SetQuitDeadlineEvent {phase,quit_deadline,});

    Ok(())
}