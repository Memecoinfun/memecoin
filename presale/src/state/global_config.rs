use anchor_lang::prelude::*;
use crate::errors::ErrorCode;

#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub create_memecoin_fee_receiver: Pubkey,
    pub launch_success_fee_receiver: Pubkey,
    pub create_memecoin_fee: u64, // default 30000000(0.03 SOL)
    pub launch_success_fee_bps: u16,  // default 175(1.75%)
}

impl GlobalConfig {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 2;

    pub fn initialize(
        &mut self,
        admin: Pubkey,
        create_memecoin_fee_receiver: Pubkey,
        create_memecoin_fee: u64,
        launch_success_fee_bps: u16,
    ) -> Result<()> {
        self.admin = admin;
        self.create_memecoin_fee_receiver = create_memecoin_fee_receiver;
        self.launch_success_fee_receiver = launch_success_fee_receiver;
        self.create_memecoin_fee = create_memecoin_fee;
        self.launch_success_fee_bps = launch_success_fee_bps;

        Ok(())
    }

    pub fn set_create_memecoin_fee_receiver(
        &mut self,
        create_memecoin_fee_receiver: &Pubkey,
    ) {
        self.create_memecoin_fee_receiver = *create_memecoin_fee_receiver;
    }

    pub fn set_launch_success_fee_receiver(
        &mut self,
        launch_success_fee_receiver: &Pubkey,
    ) {
    }

    pub fn set_create_memecoin_fee(
        &mut self,
        create_memecoin_fee: u64,
    ) -> Result<()> {
        self.create_memecoin_fee = create_memecoin_fee;

        Ok(())
    }

    pub fn set_launch_success_fee_bps(
        &mut self,
        launch_success_fee_bps: u16,
    ) -> Result<()> {
        self.launch_success_fee_bps = launch_success_fee_bps;

        Ok(())
    }
}
