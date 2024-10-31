use crate::errors::ErrorCode;
use crate::states::{QuitNft, QuitNftEvent};
use crate::utils;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction, keccak, sysvar::instructions::load_instruction_at_checked,
};
use anchor_lang::system_program;

pub fn handle_quit(ctx: Context<QuitNft>, user_storage_index: u32, nft_id: u16, nonce:u64, signature: [u8; 64], signature2: [u8; 64]) -> Result<()> {
    let collection = &mut ctx.accounts.collection.load_mut()?;
    let user_storage = &mut ctx.accounts.user_storage;

    let current_phase = collection.current_phase as usize;
    // This checks if the user is the owner of the nft_id
    require!(user_storage.is_nft_owner(nft_id as u32), ErrorCode::InvalidOwner);

    if current_phase < collection.phase_supply_max_supply.len() {
        let timestamp = Clock::get()?.unix_timestamp as u64;
        let phase_supply_quit_deadline = collection.phase_supply_quit_deadline[current_phase];
        require!(timestamp <= phase_supply_quit_deadline || phase_supply_quit_deadline == 0, ErrorCode::PhaseIsOver);
    }

    if current_phase == collection.phase_supply_max_supply.len() {
        require!(nft_id as u32 > collection.lock_max_nft_id, ErrorCode::PhaseIsOver)
    }

    // This checks if the nft_id has already been quit
    let index = (nft_id >> 3) as usize;
    let position = nft_id & 0x07;
    let bit = (collection.ids[index] >> position) & 1;
    assert_eq!(bit, 1);

    let addr_nonce = ctx.accounts.address_manager.nonce + 1;
    assert_eq!(addr_nonce, nonce);

    let mut msg = vec![];
    msg.extend(ctx.accounts.payer.key().to_bytes());
    msg.extend(nft_id.to_le_bytes());
    msg.extend(collection.current_phase.to_le_bytes());
    msg.extend(user_storage_index.to_le_bytes());
    msg.extend(nonce.to_le_bytes());

    let hash = keccak::hash(&msg).to_bytes();
    msg!("hash {}", Pubkey::new_from_array(hash));

    let ix: Instruction = load_instruction_at_checked(2, &ctx.accounts.ix_sysvar)?;
    let ix2: Instruction = load_instruction_at_checked(3, &ctx.accounts.ix_sysvar)?;
    utils::verify_ed25519_ix(&ix, &collection.oracle.to_bytes(), &hash, &signature)?;
    utils::verify_ed25519_ix(&ix2, &collection.oracle2.to_bytes(), &hash, &signature2)?;

    let input_amount = collection.price;

    let seeds: &[&[u8]] = &[b"sol_account", &[ctx.bumps.sol_account]];
    system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.sol_account.to_account_info(),
                to: ctx.accounts.payer.to_account_info(),
            },
            &[seeds],
        ),
        input_amount,
    )?;

    collection.ids[index] &= !(1 << position);
    if (nft_id as u32) <= (collection.max_supply + collection.max_supply_added) {
        collection.max_supply_added += 1
    }

    if current_phase == collection.phase_supply_current_supply.len(){
        require!(collection.phase_supply_current_supply[current_phase - 1]>=1, ErrorCode::ArrayIndexOutOfBounds);
        collection.phase_supply_current_supply[current_phase - 1] -= 1;
    } else {
        require!(collection.phase_supply_current_supply[current_phase]>=1, ErrorCode::ArrayIndexOutOfBounds);
        collection.phase_supply_current_supply[current_phase] -= 1;
    }

    msg!("Quit NFT successfully");

    ctx.accounts.address_manager.nonce = nonce;

    emit!(QuitNftEvent {
        who: ctx.accounts.payer.key(),
        nft_id,
    });

    Ok(())
}
