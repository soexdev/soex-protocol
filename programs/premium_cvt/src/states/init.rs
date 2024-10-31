use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8+8576,
        seeds = [
            "collection".as_bytes()
        ],
        bump,
    )]
    pub collection: AccountLoader<'info, CollectionAccount>,

    pub system_program: Program<'info, System>,
}


#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct CollectionAccount {
    pub admin: Pubkey,
    pub oracle: Pubkey,
    pub oracle2: Pubkey,
    pub cvt_sol_account: Pubkey,
    pub ids: [u8; 8192],
    pub max_supply: u32,
    pub current_supply: u32,
    pub price: u64,
    pub validator_nonce:u64,
    pub phase_supply_start_time: [u64;3],
    pub phase_supply_quit_deadline: [u64;3],
    pub phase_supply_max_supply: [u32;3],
    pub phase_supply_current_supply:[u32;3],
    pub phase_supply_over:[u32;3], // Safety check: Use u32 instead of bool to avoid byte alignment issues.
    pub current_phase: u32,
    pub authority:u32,
    pub lock_max_nft_id: u32,
    pub lock_mint_amount:u32,
    pub max_supply_added:u32,
    pub reserved:[u8;128],
    /* eg: Upgrade a u64 field from reversed, reversed must be placed last
    pub u64:u64,
    pub reserved:[u8;120],
    */
}

impl CollectionAccount {
    pub const MAX_SUPPLY_DESIGN_CAPACITY: u32 = (1 << 16) - 1;

    pub fn get_mint_amount(&self) -> u32 {
        self.phase_supply_current_supply[0] + self.phase_supply_current_supply[1] + self.phase_supply_current_supply[2]
    }
}

#[derive(Accounts)]
pub struct SetPhaseSupply<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: AccountLoader<'info, CollectionAccount>,
}

#[derive(Accounts)]
pub struct SetQuitDeadline<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: AccountLoader<'info, CollectionAccount>,
}

#[derive(Accounts)]
pub struct Config<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: AccountLoader<'info, CollectionAccount>,
}

#[event]
pub struct AuthorityEvent {
    pub authority: u32,
}

#[event]
pub struct SetPhaseSupplyEvent {
    pub phase: u32,
    pub start_time: u64,
    pub max_supply: u32,
}

#[event]
pub struct SetQuitDeadlineEvent {
    pub phase: u32,
    pub quit_deadline: u64,
}

#[event]
pub struct QuitDeadlineOverEvent {
    pub phase: u32,
    pub over: u32,
}

#[derive(Accounts)]
pub struct CheckQuitDeadlineOver<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub collection: AccountLoader<'info, CollectionAccount>,
}