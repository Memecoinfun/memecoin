use anchor_lang::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEvent {
    
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: String,
    pub memecoin_config: String,
    pub creator: String,
    pub created_time: u64,
    pub destination: String,
    pub description: String,
    pub website: String,
    pub telegram : String,
    pub twitter: String,
    pub decimal: u8,
}




#[derive(Serialize, Deserialize, Debug)]
pub struct TradeEvent {
    pub mint: String,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: String,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,

    pub hash : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteEvent {
    pub user: String,
    pub mint: String,
    pub bonding_curve: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetParamsEvent {
    pub fee_recipient: Pubkey,
    pub withdraw_authority: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub initial_token_supply: u64,
    pub fee_basis_points: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ClaimInviteProfitEvent {
    pub user: String,
    pub amount: u64,
    pub timestamp: i64,
}