use anchor_lang::prelude::*;
use num_bigint::BigInt;

extern crate num_bigint;
extern crate num_traits;

use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::solana_program::{
    clock::UnixTimestamp, program::invoke, system_instruction::transfer as lamports_transfer,
    sysvar::clock::Clock,
};
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token::{transfer as memecoin_transfer, Burn, Mint, Token, TokenAccount, Transfer},
    //token_2022::{self, transfer_checked as memecoin_transfer, TransferChecked, Token2022},
};
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use serde::{Deserialize, Serialize};

#[derive(Accounts)]
pub struct BuyMemecoin<'info> {
    #[account(
    mut,
    seeds = [memecoin_config.creator.key().as_ref(), & memecoin_config.creator_memecoin_index.to_le_bytes()],
    bump
    )]
    pub memecoin_config: Account<'info, MemecoinConfig>,

    #[account(
    mut,
    address = memecoin_config.mint.key()
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
    init_if_needed,
    payer = buyer,
    associated_token::mint = mint,
    associated_token::authority = buyer
    )]
    pub buyer_token: Account<'info, TokenAccount>,

    #[account(
    mut,
    token::mint = mint,
    token::authority = memecoin_config,
    seeds = [b"MEME_COIN", mint.key().as_ref(), memecoin_config.key().as_ref()],
    bump
    )]
    pub memecoin_config_token: Account<'info, TokenAccount>,

    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// Spl token program or token program 2022
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[event]
#[derive(Serialize, Deserialize, Debug)]
pub struct MemecoinBought {
    pub buyer: String,
    pub buy_amount: u64,
    pub mint: String,
    pub token_price: u64,
    pub remain_amount: u64,
    // Remaining amount to sell
    pub hash: String,
}

pub fn handler(ctx: Context<BuyMemecoin>, hash: &str, buy_sol_amount: u64) -> Result<()> {
    require!(
        ctx.accounts.memecoin_config.status == LaunchStatus::Ongoing,
        ErrorCode::StatusNotOngoing
    );
    let memecoin_total_sold = MEMECOIN_TOTAL_SUPPLY * 7 / 10;
    let memecoin_config: &mut Account<MemecoinConfig> = &mut ctx.accounts.memecoin_config;
    let total_sold: BigUint = BigUint::from(memecoin_total_sold);
    let buy_sol_amount_uint: BigUint = BigUint::from(buy_sol_amount);
    let buy_amount = (total_sold * buy_sol_amount_uint
        / BigUint::from(memecoin_config.funding_raise_tier.value()))
        .to_u64()
        .expect("too large for convert");
    msg!("sold {} memecoin", buy_amount);
    let memecoin_config_token_balance = ctx.accounts.memecoin_config_token.amount;

    let sold_amount = MEMECOIN_TOTAL_SUPPLY
        .checked_sub(memecoin_config_token_balance)
        .ok_or_else(|| ErrorCode::CalculationError)?;
    require!(
        sold_amount + buy_amount <= (memecoin_total_sold),
        ErrorCode::UnsoldTokenInsufficient
    );

    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    let memecoin_created_time = memecoin_config.created_time;
    if current_timestamp >= memecoin_created_time + memecoin_config.funding_raise_tier.time() {
        if sold_amount == (memecoin_total_sold) {
            memecoin_config.set_memecoin_status(LaunchStatus::Succeed)?;
        } else {
            memecoin_config.set_memecoin_status(LaunchStatus::Failed)?;
        }

        return err!(ErrorCode::SaleClosed);
    } else {
        if sold_amount + buy_amount == (memecoin_total_sold) {
            memecoin_config.set_memecoin_status(LaunchStatus::Succeed)?;
        }
    }

    // User pay for the memecoin by lamports
    let token_price = ctx.accounts.memecoin_config.token_price()?;
    let cost = buy_sol_amount;
    let transfer_instruction = lamports_transfer(
        &ctx.accounts.buyer.key(),
        &ctx.accounts.memecoin_config.key(),
        cost,
    );
    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.memecoin_config.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Send user the memecoin
    let seeds = &[
        ctx.accounts.memecoin_config.creator.as_ref(),
        &ctx.accounts
            .memecoin_config
            .creator_memecoin_index
            .to_le_bytes(),
        &[ctx.bumps.memecoin_config],
    ];
    let signer = [&seeds[..]];

    memecoin_transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.memecoin_config_token.to_account_info(),
                to: ctx.accounts.buyer_token.to_account_info(),
                authority: ctx.accounts.memecoin_config.to_account_info(),
            },
            &signer,
        ),
        buy_amount,
    )?;

    let remain_amount = (MEMECOIN_TOTAL_SUPPLY * 7 / 10)
        .checked_sub(sold_amount)
        .unwrap()
        .checked_sub(buy_amount)
        .unwrap();
    let event = MemecoinBought {
        buyer: ctx.accounts.buyer.key().to_string(),
        buy_amount,
        mint: ctx.accounts.mint.key().to_string(),
        token_price,
        remain_amount,
        hash: hash.to_string(),
    };
    let serialized = serde_json::to_string(&event).unwrap();
    msg!("===================================");
    msg!("buylog:{}", serialized);
    msg!("===================================");
    emit!(event);

    Ok(())
}
