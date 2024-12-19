use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct Global {
    pub authority: Pubkey,
    pub initialized: bool,
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub initial_real_sol_reserves: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Pubkey,

    pub creator_fee_basis_points: u64,
    pub protocol_token_alloc_points:u64,
    pub protocol_token_alloc_recipient:Pubkey,

    pub invite_fee_basis_points: u64,
}

impl Global {
   pub const SEED_PREFIX: &'static [u8; 6] = b"CONFIG";
}