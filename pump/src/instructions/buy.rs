use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    amm, calculate_fee,
    state::{BondingCurve, FeeAccount, Global, UserInviteStats},
    CompleteEvent, CurveLaunchpadError, TradeEvent,
};

#[derive(Accounts)]
pub struct Buy<'info> {
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
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,

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
    associated_token_program: Program<'info, AssociatedToken>,

}

pub fn buy(ctx: Context<Buy>, token_amount: u64, max_sol_cost: u64, hash: &str) -> Result<()> {
    //bonding curve is not complete
    require!(
        ctx.accounts.bonding_curve.complete == false,
        CurveLaunchpadError::BondingCurveComplete,
    );

    require!(ctx.accounts.user_invite_account.is_init,CurveLaunchpadError::InviteAccountError);

    //bonding curve has enough tokens
    require!(
        ctx.accounts.bonding_curve.real_token_reserves >= token_amount,
        CurveLaunchpadError::InsufficientTokens,
    );

    require!(token_amount > 0, CurveLaunchpadError::MinBuy,);

    let targe_token_amount = if ctx.accounts.bonding_curve_token_account.amount < token_amount {
        ctx.accounts.bonding_curve_token_account.amount
    } else {
        token_amount
    };

    let mut amm = amm::amm::AMM::new(
        ctx.accounts.bonding_curve.virtual_sol_reserves as u128,
        ctx.accounts.bonding_curve.virtual_token_reserves as u128,
        ctx.accounts.global.initial_virtual_token_reserves as u128,
    );

    let buy_result = amm.apply_buy(targe_token_amount as u128).unwrap();
    let fee = calculate_fee(
        buy_result.sol_amount,
        ctx.accounts.global.fee_basis_points + ctx.accounts.global.creator_fee_basis_points + ctx.accounts.global.invite_fee_basis_points,
    );
    let buy_amount_with_fee = buy_result.sol_amount + fee;


    //check if the amount of SOL to transfe plus fee is less than the max_sol_cost
    require!(
        buy_amount_with_fee <= max_sol_cost,
        CurveLaunchpadError::MaxSOLCostExceeded,
    );

    //check if the user has enough SOL
    require!(
        ctx.accounts.user.lamports() >= buy_amount_with_fee,
        CurveLaunchpadError::InsufficientSOL,
    );

    // transfer SOL to bonding curve
    let from_account = &ctx.accounts.user;
    let to_bonding_curve_account = &ctx.accounts.bonding_curve;

    let transfer_instruction = system_instruction::transfer(
        from_account.key,
        to_bonding_curve_account.to_account_info().key,
        buy_result.sol_amount,
    );

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            to_bonding_curve_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    //transfer SOL to fee account

    let transfer_instruction =
        system_instruction::transfer(from_account.key, ctx.accounts.fee_account.to_account_info().key, fee);

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            from_account.to_account_info(),
            ctx.accounts.fee_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    ctx.accounts.fee_account.received += fee;
    require!(is_ok,CurveLaunchpadError::FeeAccountStatusAbnormal);

    
    let protocol_fee = calculate_fee(buy_result.sol_amount, ctx.accounts.global.fee_basis_points);
    let creator_fee = calculate_fee(buy_result.sol_amount, ctx.accounts.global.creator_fee_basis_points);


    ctx.accounts.fee_recipient_invite_account.profit_claimable += protocol_fee;
    ctx.accounts.creator_fee_recipient_invite_account.profit_claimable += creator_fee;
    ctx.accounts.parent_invite_account.profit_claimable += invite_fee;

    ctx.accounts.user_invite_account.profit_to_parent += invite_fee;
    ctx.accounts.parent_invite_account.profit_from_child += invite_fee;



    //transfer SPL
    let cpi_accounts = Transfer {
        from: ctx
            .accounts
            .bonding_curve_token_account
            .to_account_info()
            .clone(),
        to: ctx.accounts.user_token_account.to_account_info().clone(),
    };

    let signer: [&[&[u8]]; 1] = [&[
        BondingCurve::SEED_PREFIX,
        ctx.accounts.mint.to_account_info().key.as_ref(),
    ]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        buy_result.token_amount,
    )?;

    //apply the buy to the bonding curve
    let bonding_curve = &mut ctx.accounts.bonding_curve;
    bonding_curve.real_token_reserves = amm.real_token_reserves as u64;
    bonding_curve.real_sol_reserves = amm.real_sol_reserves as u64;
    bonding_curve.virtual_token_reserves = amm.virtual_token_reserves as u64;
    bonding_curve.virtual_sol_reserves = amm.virtual_sol_reserves as u64;

    let curr_time = Clock::get()?.unix_timestamp;
    bonding_curve.update_time = curr_time as u64;

    let trade_event = TradeEvent {
        mint: ctx.accounts.mint.to_account_info().key().to_string(),
        sol_amount: buy_result.sol_amount,
        token_amount: buy_result.token_amount,
        virtual_sol_reserves: bonding_curve.virtual_sol_reserves,
        virtual_token_reserves: bonding_curve.virtual_token_reserves,
        real_sol_reserves: bonding_curve.real_sol_reserves,
        real_token_reserves: bonding_curve.real_token_reserves,
        hash: hash.to_string(),
    };

    let serialized = serde_json::to_string(&trade_event).unwrap();

    msg!("tradelog:{}", serialized);

    if bonding_curve.real_token_reserves == 0 {

        let complete_event = CompleteEvent {
            user: ctx.accounts.user.to_account_info().key().to_string(),
            mint: ctx.accounts.mint.to_account_info().key().to_string(),
            bonding_curve: ctx
                .accounts
                .bonding_curve
                .to_account_info()
                .key()
                .to_string(),
            timestamp: curr_time,
        };
        let serialized = serde_json::to_string(&complete_event).unwrap();
        msg!("completelog:{}", serialized);
    }

    msg!("bonding_curve: {:?}", amm);

    Ok(())
}
