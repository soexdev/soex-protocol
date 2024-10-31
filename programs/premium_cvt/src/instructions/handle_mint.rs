use crate::errors::ErrorCode;
use crate::states::{CollectionAccount, MintNft, MintNftEvent, UserStorageMinted};
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::config::QUIT_DEADLINE_DURATION;

// Safety check: user_storage_index is used to generate PDA accounts

pub fn handle_mint(ctx: Context<MintNft>, user_storage_index: u32, copies: u32, phase: u32) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;

    let user_storage = &mut ctx.accounts.user_storage;
    require!(phase < collection.phase_supply_max_supply.len() as u32, ErrorCode::InvalidPhase);
    require!(copies > 0 && copies <= 10, ErrorCode::InvalidCopies);
    let is_last_phase = phase == (collection.phase_supply_max_supply.len() - 1) as u32;
    if !is_last_phase {
        require!((copies + collection.phase_supply_current_supply[phase as usize]) <= collection.phase_supply_max_supply[phase as usize],ErrorCode::MaxSupplyReached);
    }
    require!((collection.authority&0x01)==1, ErrorCode::InvalidAuthority);

    let timestamp = Clock::get()?.unix_timestamp as u64;
    let phase_supply_start_time = collection.phase_supply_start_time[phase as usize];
    let phase_supply_quit_deadline = collection.phase_supply_quit_deadline[phase as usize];
    msg!("handle_mint timestamp {} start_time {}", timestamp, phase_supply_start_time);
    require!(timestamp >= phase_supply_start_time && phase_supply_start_time > 0, ErrorCode::PhaseNotStarted);
    require!(timestamp <= phase_supply_quit_deadline || phase_supply_quit_deadline==0, ErrorCode::PhaseIsOver);

    let input_amount = copies as u64 * collection.price;

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.payer.to_account_info().clone(),
            to: ctx.accounts.sol_account.to_account_info().clone(),
        },
    );
    system_program::transfer(cpi_context, input_amount)?;

    let mut quit_deadline = 0;
    let current = collection.current_supply;
    for _i in 0..copies {
        collection.current_supply += 1;
        require!(collection.current_supply <= CollectionAccount::MAX_SUPPLY_DESIGN_CAPACITY, ErrorCode::MaxSupplyReached);

        collection.phase_supply_current_supply[phase as usize] += 1;
        // last phase manual lock
        if !is_last_phase  &&collection.phase_supply_quit_deadline[phase as usize] == 0 &&
            collection.phase_supply_current_supply[phase as usize] == collection.phase_supply_max_supply[phase as usize] {
            quit_deadline = timestamp + QUIT_DEADLINE_DURATION;
            collection.phase_supply_quit_deadline[phase as usize] = quit_deadline;
        }
        let nft_id = collection.current_supply;

        let index = (nft_id >> 3) as usize;
        let position = nft_id & 0x07;
        let bit = (collection.ids[index] >> position) & 1;
        assert_eq!(bit, 0);
        collection.ids[index] |= 1 << position;
    }

    let position = user_storage.position;
    require!(position < user_storage.minted.len() as u32, ErrorCode::ArrayIndexOutOfBounds);
    user_storage.minted[position as usize] =  UserStorageMinted{
        current,
        copies,
    };
    user_storage.position += 1;

    msg!("Minted NFT successfully");

    emit!(MintNftEvent {
        who: ctx.accounts.payer.key(),
        current,
        copies,
        user_storage_index,
        phase,
        quit_deadline,
    });

    Ok(())
}
