use crate::{state::{FeeAccount, Global}, CurveLaunchpadError, DEFAULT_TOKEN_SUPPLY};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        init,
        bump,
        payer = authority,
    )]
    global: Box<Account<'info, Global>>,


    #[account(
        init,
        space = 8 + FeeAccount::INIT_SPACE,
        payer = authority,
        seeds=[FeeAccount::SEED_PREFIX],
        bump
    )]
    fee_account: Box<Account<'info, FeeAccount>>,

    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let global = &mut ctx.accounts.global;

    require!(!global.initialized, CurveLaunchpadError::AlreadyInitialized,);

    global.authority = *ctx.accounts.authority.to_account_info().key;
    global.withdraw_authority = *ctx.accounts.authority.to_account_info().key;
    global.fee_recipient = *ctx.accounts.authority.to_account_info().key;
    global.initialized = true;
    global.initial_token_supply = DEFAULT_TOKEN_SUPPLY;
    global.initial_virtual_token_reserves = 1_075_000_000_000_000;
    global.fee_basis_points = 50;

    global.creator_fee_basis_points = 35;
    global.protocol_token_alloc_points = 50;
    global.protocol_token_alloc_recipient = *ctx.accounts.authority.to_account_info().key;
    global.invite_fee_basis_points = 15;

    msg!("Initialized global state");

    Ok(())
}
