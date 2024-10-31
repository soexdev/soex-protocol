use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

pub fn handle_set_phase_supply_start_time(ctx: Context<SetPhaseSupply>, phase:u32, start_time: u64) -> anchor_lang::Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(collection.admin == ctx.accounts.payer.key(), ErrorCode::InvalidAdmin);
    require!(phase<collection.phase_supply_start_time.len() as u32, ErrorCode::InvalidPhase);
    require!(phase == 0 || (start_time >= collection.phase_supply_quit_deadline[(phase - 1) as usize] && collection.phase_supply_quit_deadline[(phase - 1) as usize]>0), ErrorCode::InvalidPhase);
    let timestamp = Clock::get()?.unix_timestamp as u64;
    assert!(start_time>=timestamp);

    collection.phase_supply_start_time[phase as usize] = start_time;

    msg!("set_phase_supply_start_time {} {}", phase, start_time);
    emit!(SetPhaseSupplyEvent {phase, start_time, max_supply: collection.phase_supply_max_supply[phase as usize] as u32});

    Ok(())
}