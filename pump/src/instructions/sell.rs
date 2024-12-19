use crate::{
    amm, calculate_fee, state::{BondingCurve, FeeAccount, Global, UserInviteStats}, CurveLaunchpadError, TradeEvent
};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [Global::SEED_PREFIX],
        bump,
    )]
    global: Box<Account<'info, Global>>,


    #[account(
        mut,
        address = bonding_curve.mint.key()
    )]
    mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX, mint.to_account_info().key.as_ref()],
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = bonding_curve,
        seeds=[BondingCurve::SEED_PREFIX, mint.key().as_ref(), bonding_curve.key().as_ref()],
        bump
    )]
    bonding_curve_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,




    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,user.key().as_ref()],
        bump
    )]
    user_invite_account: Box<Account<'info, UserInviteStats>>,

    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,user_invite_account.parent.key().as_ref()],
        bump
    )]
    parent_invite_account: Box<Account<'info, UserInviteStats>>,

    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,global.fee_recipient.key().as_ref()],
        bump
    )]
    fee_recipient_invite_account: Box<Account<'info,UserInviteStats>>,

    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,bonding_curve.creator.key().as_ref()],
        bump
    )]
    creator_fee_recipient_invite_account: Box<Account<'info,UserInviteStats>>,


    #[account(
        mut,
        seeds=[FeeAccount::SEED_PREFIX],
        bump
    )]
    fee_account : Box<Account<'info,FeeAccount>>,


    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,
}

pub fn sell(ctx: Context<Sell>, token_amount: u64, min_sol_output: u64 , hash : &str) -> Result<()> {
    //check if bonding curve is complete
    require!(
        !ctx.accounts.bonding_curve.complete,
        CurveLaunchpadError::BondingCurveComplete,
    );

    require!(ctx.accounts.user_invite_account.is_init,CurveLaunchpadError::InviteAccountError);


    //confirm user has enough tokens
    require!(
        ctx.accounts.user_token_account.amount >= token_amount,
        CurveLaunchpadError::InsufficientTokens,
    );

    //confirm bonding curve has enough tokens
    require!(
        ctx.accounts.bonding_curve_token_account.amount >= token_amount,
        CurveLaunchpadError::InsufficientTokens,
    );

    require!(token_amount > 0, CurveLaunchpadError::MinSell,);

    let mut amm = amm::amm::AMM::new(
        ctx.accounts.bonding_curve.virtual_sol_reserves as u128,
        ctx.accounts.bonding_curve.virtual_token_reserves as u128,
        ctx.accounts.bonding_curve.real_sol_reserves as u128,
        ctx.accounts.bonding_curve.real_token_reserves as u128,
        ctx.accounts.global.initial_virtual_token_reserves as u128,
    );

    let sell_result = amm.apply_sell(token_amount as u128).unwrap();
    let fee = calculate_fee(
        sell_result.sol_amount,
        ctx.accounts.global.fee_basis_points + ctx.accounts.global.creator_fee_basis_points + ctx.accounts.global.invite_fee_basis_points,
    );
    //the fee is subtracted from the sol amount to confirm the user minimum sol output is met
    let sell_amount_minus_fee = sell_result.sol_amount - fee;

    //confirm min sol output is greater than sol output
    require!(
        sell_amount_minus_fee >= min_sol_output,
        CurveLaunchpadError::MinSOLOutputExceeded,
    );

    //transfer SPL
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info().clone(),
        to: ctx
            .accounts
            .bonding_curve_token_account
            .to_account_info()
            .clone(),
        authority: ctx.accounts.user.to_account_info().clone(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &[],
        ),
        sell_result.token_amount,
    )?;



    //transfer SOL back to user
    let from_account = &ctx.accounts.bonding_curve;
    let to_account = &ctx.accounts.user;
    let fee_account = &ctx.accounts.fee_account;

    **from_account.to_account_info().try_borrow_mut_lamports()? -= sell_result.sol_amount;
    **to_account.try_borrow_mut_lamports()? += sell_amount_minus_fee;
    **fee_account.to_account_info().try_borrow_mut_lamports()? += fee;



    ctx.accounts.fee_account.received += fee;
    let is_ok = ctx.accounts.fee_account.check(ctx.accounts.fee_account.get_lamports());
    require!(is_ok,CurveLaunchpadError::FeeAccountStatusAbnormal);

    
    let protocol_fee = calculate_fee(sell_result.sol_amount, ctx.accounts.global.fee_basis_points);
    let creator_fee = calculate_fee(sell_result.sol_amount, ctx.accounts.global.creator_fee_basis_points);
    let invite_fee = calculate_fee(sell_result.sol_amount, ctx.accounts.global.invite_fee_basis_points);


    ctx.accounts.fee_recipient_invite_account.profit_claimable += protocol_fee;
    ctx.accounts.creator_fee_recipient_invite_account.profit_claimable += creator_fee;
    ctx.accounts.parent_invite_account.profit_claimable += invite_fee;

    ctx.accounts.user_invite_account.profit_to_parent += invite_fee;
    ctx.accounts.parent_invite_account.profit_from_child += invite_fee;



    let bonding_curve = &mut ctx.accounts.bonding_curve;
    bonding_curve.real_token_reserves = amm.real_token_reserves as u64;
    bonding_curve.real_sol_reserves = amm.real_sol_reserves as u64;
    bonding_curve.virtual_token_reserves = amm.virtual_token_reserves as u64;
    bonding_curve.virtual_sol_reserves = amm.virtual_sol_reserves as u64;

    let trade_event = TradeEvent {
        mint: ctx.accounts.mint.to_account_info().key().to_string(),
        sol_amount: sell_result.sol_amount,
        token_amount: sell_result.token_amount,
        is_buy: false,
        user: ctx.accounts.user.to_account_info().key().to_string(),
        timestamp: Clock::get()?.unix_timestamp,
        virtual_sol_reserves: bonding_curve.virtual_sol_reserves,
        virtual_token_reserves: bonding_curve.virtual_token_reserves,
        real_sol_reserves: bonding_curve.real_sol_reserves,
        real_token_reserves: bonding_curve.real_token_reserves,

        hash: hash.to_string(),
    };

    let serialized = serde_json::to_string(&trade_event).unwrap();

    msg!("tradelog:{}", serialized);

    Ok(())
}
