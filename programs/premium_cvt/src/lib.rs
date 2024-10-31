use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod states;
mod utils;
mod config;

use crate::config::*;
use crate::errors::ErrorCode;
use instructions::*;
use states::*;

declare_id!("J7uhg7UDfvSEZHgZDrwp7SqFrejZiTvQWvacAnxnouS");

#[program]
pub mod premium_cvt {
    use super::*;

    pub fn init(
        ctx: Context<Initialize>,
        admin: Pubkey,
        creater: Pubkey,
        creater2: Pubkey,
        max_supply: u32,
        phase0_max_supply: u32,
        phase1_max_supply: u32,
        phase2_max_supply: u32,
    ) -> Result<()> {
        require!(max_supply == (phase0_max_supply + phase1_max_supply + phase2_max_supply), ErrorCode::InvalidMaxSupply);
        require!(ctx.accounts.payer.key().to_string() == ADMIN_PUBKEY, ErrorCode::AdminAccessRequired);
        let collection = &mut ctx.accounts.collection.load_init()?;
        collection.admin = admin;
        collection.max_supply = max_supply;
        collection.oracle = creater;
        collection.oracle2 = creater2;
        collection.price = PREMIUM_CVT_PRICE;
        collection.authority = 0x01;
        collection.phase_supply_max_supply[0] = phase0_max_supply;
        collection.phase_supply_max_supply[1] = phase1_max_supply;
        collection.phase_supply_max_supply[2] = phase2_max_supply;
        Ok(())
    }

    pub fn check_quit_deadline_over(ctx: Context<CheckQuitDeadlineOver>, phase:u32) -> Result<()> {
        handle_check_quit_deadline_over(ctx, phase)
    }

    pub fn set_phase_supply_start_time(ctx: Context<SetPhaseSupply>, phase:u32, start_time: u64) -> Result<()> {
        handle_set_phase_supply_start_time(ctx, phase, start_time)
    }

    pub fn set_phase_supply_quit_deadline(ctx: Context<SetPhaseSupply>, phase:u32, quit_deadline:u64) -> Result<()> {
        handle_set_phase_supply_quit_deadline(ctx, phase, quit_deadline)
    }

    pub fn init_user_storage(
        ctx: Context<InitUserStorage>,
        user_storage_index: u32,
    ) -> Result<()> {
        msg!("Initializing user storage with index: {} and PDA: {}", user_storage_index,ctx.accounts.user_storage.key());
        Ok(())
    }

    pub fn init_address_manager(ctx:Context<InitAddressManager>)-> Result<()> {
        msg!("init_address_manager");
        Ok(())
    }

    pub fn mint(ctx: Context<MintNft>, user_storage_index: u32, copies: u32, phase: u32) -> Result<()> {
        handle_mint(ctx, user_storage_index, copies, phase)
    }

    pub fn quit(ctx: Context<QuitNft>, user_storage_index: u32, nft_id: u16, nonce:u64, signature: [u8; 64], signature2: [u8; 64]) -> Result<()> {
        handle_quit(ctx, user_storage_index, nft_id, nonce, signature,signature2)
    }

    pub fn transfer(ctx: Context<Transfer>) -> Result<()> {
        handle_transfer(ctx)
    }

    pub fn set_max_supply(ctx: Context<Config>, new_max_supply: u32) -> Result<()> {
        let collection = &mut ctx.accounts.collection.load_mut()?;
        require!(
            collection.admin == ctx.accounts.payer.key(),
            ErrorCode::InvalidAdmin
        );
        assert!(new_max_supply >= collection.current_supply);
        collection.max_supply = new_max_supply;
        msg!("Max supply set to {}", new_max_supply);
        Ok(())
    }

    pub fn set_authority(ctx: Context<Config>, authority: u32) -> Result<()> {
        handle_set_authority(ctx, authority)
    }

    pub fn set_manager(ctx: Context<SetManager>, manager: Pubkey) -> Result<()> {
        handle_set_manager(ctx, manager)
    }

    pub fn set_oracle(ctx: Context<SetOracle>, oracle: Pubkey,nonce:u64,signature: [u8; 64]) -> Result<()> {
        handle_set_oracle(ctx, oracle,nonce,signature)
    }

    pub fn set_oracle2(ctx: Context<SetOracle>, oracle: Pubkey,nonce:u64,signature: [u8; 64]) -> Result<()> {
        handle_set_oracle2(ctx, oracle,nonce,signature)
    }

    pub fn set_cvt_sol_account(ctx: Context<SetCvtSolAccount>, cvt_sol_account: Pubkey, nonce:u64, signature: [u8; 64]) -> Result<()> {
        handle_set_cvt_sol_account(ctx, cvt_sol_account, nonce, signature)
    }

    pub fn verify_cvt_sol_account(ctx: Context<Transfer>) -> Result<()> {
        handle_verify_cvt_sol_account(ctx)
    }

    pub fn get_version(ctx: Context<GetVersion>) -> Result<String> {
        handle_get_version(ctx)
    }

    pub fn check_version(ctx: Context<CheckVersion>, code:String,nonce:u64, signature: [u8; 64]) ->Result<()> {
        handle_check_version(ctx, code, nonce,signature)
    }

    pub fn init_version_account(ctx: Context<InitVersion>) ->Result<()> {
        handle_init_version_account(ctx )
    }
}
