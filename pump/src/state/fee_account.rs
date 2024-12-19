use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct FeeAccount {
    pub received : u64,
    pub sent : u64,
}

impl FeeAccount {
   pub const SEED_PREFIX: &'static [u8; 3] = b"FEE";

   pub fn check(&self,balance: u64)->bool{
        let expected_balance = self.received - self.sent;
        balance >= expected_balance
   }
}