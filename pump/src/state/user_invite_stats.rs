use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserInviteStats {

    pub key: Pubkey,
    pub parent: Pubkey,
    pub child_count: u64,
    pub profit_from_child: u64,
    pub profit_to_parent: u64,
    pub profit_claimable: u64,
    pub profit_claim_accumulated: u64,
    pub is_init: bool,
}

impl UserInviteStats {
    pub const SEED_PREFIX: &'static [u8; 17] = b"user-invite-stats";
}


