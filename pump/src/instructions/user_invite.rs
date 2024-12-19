use anchor_lang::prelude::*;

use crate::state::{FeeAccount, UserInviteStats};
use crate::{ClaimInviteProfitEvent, CurveLaunchpadError};



#[derive(Accounts)]
#[instruction(parent: Pubkey)]
pub struct InitInviteAccount<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init_if_needed,
        space = 8 + UserInviteStats::INIT_SPACE,
        payer = user,
        seeds=[UserInviteStats::SEED_PREFIX,user.key().as_ref()],
        bump
    )]
    user_invite_account: Box<Account<'info, UserInviteStats>>,

    #[account(
        init_if_needed,
        space = 8 + UserInviteStats::INIT_SPACE,
        payer = user,
        seeds=[UserInviteStats::SEED_PREFIX,parent.as_ref()],
        bump
    )]
    parent_invite_account: Box<Account<'info, UserInviteStats>>,

    system_program: Program<'info, System>,
}

pub fn init_invite(ctx: Context<InitInviteAccount>, parent: Pubkey) -> Result<()> {


    require_keys_neq!(ctx.accounts.user_invite_account.key(), ctx.accounts.parent_invite_account.key(), CurveLaunchpadError::InviteAccountError);


    if ctx.accounts.user_invite_account.is_init {
        return Ok(());
    }
    ctx.accounts.user_invite_account.is_init = true;
    if !ctx.accounts.parent_invite_account.is_init {
        ctx.accounts.parent_invite_account.is_init = true;
    }

    ctx.accounts.user_invite_account.key = ctx.accounts.user.key();
    ctx.accounts.user_invite_account.parent = parent;
    ctx.accounts.parent_invite_account.child_count += 1;

    Ok(())
}




#[derive(Accounts)]
pub struct InviteClaimAccount<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,user.key().as_ref()],
        bump
    )]
    user_invite_account: Box<Account<'info, UserInviteStats>>,


    #[account(
        mut,
        seeds=[FeeAccount::SEED_PREFIX],
        bump
    )]
    fee_account : Box<Account<'info,FeeAccount>>,
}


pub fn claim_inivte_profit(ctx: Context<InviteClaimAccount>) -> Result<()> {

    let claim_amount = ctx.accounts.user_invite_account.profit_claimable;
    require!(claim_amount > 0,CurveLaunchpadError::NotClaimableFee);
    ctx.accounts.user_invite_account.profit_claimable = 0;
    ctx.accounts.user_invite_account.profit_claim_accumulated += claim_amount;

    let fee_account = &ctx.accounts.fee_account;
    let user_account = &ctx.accounts.user;


    **fee_account.to_account_info().try_borrow_mut_lamports()? -= claim_amount;
    **user_account.try_borrow_mut_lamports()? += claim_amount;

    ctx.accounts.fee_account.sent += claim_amount;
    let is_ok = ctx.accounts.fee_account.check(ctx.accounts.fee_account.get_lamports());
    require!(is_ok,CurveLaunchpadError::FeeAccountStatusAbnormal);
    

    let claim_event = ClaimInviteProfitEvent{
        user: ctx.accounts.user.to_account_info().key().to_string(),
        amount: claim_amount,
        timestamp: Clock::get()?.unix_timestamp,
    };

    let serialized = serde_json::to_string(&claim_event).unwrap();

    msg!("claimInviteProfit:{}", serialized);


    Ok(())
}