use crate::{state::Global, CurveLaunchpadError};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetParams<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [Global::SEED_PREFIX],
        bump,
    )]

    #[account(mut)]
    authority: Signer<'info>,

    system_program: Program<'info, System>,
}


pub fn set_amm_params(
    ctx: Context<SetParams>,
    initial_virtual_token_reserves: u64,
    initial_virtual_sol_reserves: u64,
    initial_real_token_reserves: u64,
    initial_token_supply: u64,

) -> Result<()> {
    let global = &mut ctx.accounts.global;
    require!(global.initialized, CurveLaunchpadError::NotInitialized);

    global.initial_virtual_token_reserves = initial_virtual_token_reserves;
    global.initial_virtual_sol_reserves = initial_virtual_sol_reserves;
    global.initial_real_token_reserves = initial_real_token_reserves;
    global.initial_token_supply = initial_token_supply;

    Ok(())
}

pub fn set_fee_params(
    ctx: Context<SetParams>,
    fee_recipient: Pubkey,
    fee_basis_points: u64,
    creator_fee_basis_points: u64,
    withdraw_authority: Pubkey,
    invite_fee_basis_points: u64,
) -> Result<()> {
    let global = &mut ctx.accounts.global;
    require!(global.initialized, CurveLaunchpadError::NotInitialized);

    global.fee_recipient = fee_recipient;
    global.fee_basis_points = fee_basis_points;
    global.invite_fee_basis_points = invite_fee_basis_points;

    Ok(())
}



pub fn set_protocol_fee_address(ctx: Context<SetParams>,protocol_token_alloc_recipient: Pubkey,fee_recipient: Pubkey) -> Result<()> {

    ctx.accounts.global.protocol_token_alloc_recipient = protocol_token_alloc_recipient;
    ctx.accounts.global.fee_recipient = fee_recipient;

    Ok(())
}