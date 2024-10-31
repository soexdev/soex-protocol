use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::ErrorCode;

pub fn handle_check_quit_deadline_over(ctx: Context<CheckQuitDeadlineOver>, phase:u32) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    require!(phase<collection.phase_supply_quit_deadline.len() as u32, ErrorCode::InvalidPhase);
    require!(collection.phase_supply_over[phase as usize] == 0, ErrorCode::PhaseIsOver);
    require!(collection.current_phase == phase, ErrorCode::InvalidPhase);

    let quit_deadline = collection.phase_supply_quit_deadline[phase as usize];
    let timestamp = Clock::get()?.unix_timestamp as u64;

    let over = if timestamp >= quit_deadline && quit_deadline > 0 { 1 } else { 0 };
    require!(over == 1, ErrorCode::InvalidAction);
    if phase > 0 {
        require!(collection.phase_supply_over[(phase -1) as usize] == 1, ErrorCode::InvalidPhase);
    }

    collection.phase_supply_over[phase as usize] = over;
    collection.current_phase += 1;

    require!(collection.current_phase <= collection.phase_supply_over.len() as u32, ErrorCode::InvalidAction);
    if collection.current_phase == collection.phase_supply_over.len() as u32 {
        let mint_amount = collection.get_mint_amount();
        if mint_amount > collection.max_supply {
            collection.lock_max_nft_id = collection.max_supply + collection.max_supply_added;
            collection.lock_mint_amount = collection.max_supply;
        } else {
            collection.lock_max_nft_id = mint_amount + collection.max_supply_added;
            collection.lock_mint_amount = mint_amount;
        }

        msg!("set_lock_mint_amount {}", collection.lock_mint_amount);
        msg!("set_lock_max_nft_id {}", collection.lock_max_nft_id);

        collection.authority &= !0x01u32;
        msg!("set_authority {}", collection.authority);
        emit!(AuthorityEvent {authority:collection.authority});
    }

    msg!("check_quit_deadline_over {} {}", phase, over);
    emit!(QuitDeadlineOverEvent{phase, over });

    Ok(())
}