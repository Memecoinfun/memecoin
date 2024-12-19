use anchor_lang::prelude::*;
use crate::state::*;
use anchor_lang::{
    solana_program::{
        clock::UnixTimestamp,
        sysvar::clock::Clock,
    },
    system_program,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{self, Token2022, Transfer, transfer_checked},
    token::{self, Token, TokenAccount, Mint}
};
use crate::constants::WSOL_MINT_ADDRESS;
use crate::errors::ErrorCode;
use std::str::FromStr;


#[derive(Accounts)]
pub struct WrapSolSyncNative<'info> {
    #[account(
        seeds = [memecoin_config.creator.key().as_ref(), &memecoin_config.creator_memecoin_index.to_le_bytes()],
        bump
    )]
    pub memecoin_config: Account<'info, MemecoinConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK：checked in the handler
    pub wrapped_sol_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        token::mint = wrapped_sol_mint,
        token::authority = memecoin_config,
        seeds=[b"WSOL", memecoin_config.key().as_ref()],
        bump
    )]
    pub memecoin_config_wrapped_sol_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WrapSolSyncNative>,
) -> Result<()> {
    let wsol_mint_pubkey = Pubkey::from_str(WSOL_MINT_ADDRESS).unwrap();
    require_keys_eq!(ctx.accounts.wrapped_sol_mint.key(), wsol_mint_pubkey, ErrorCode::WrongWSOLMint);

    let seeds = &[
        ctx.accounts.memecoin_config.creator.as_ref(),
        &ctx.accounts.memecoin_config.creator_memecoin_index.to_le_bytes(),
        &[ctx.bumps.memecoin_config]
    ];
    let signer = [&seeds[..]];

    // Sync the native token to reflect the new SOL balance as wSOL
    token::sync_native(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::SyncNative {
                account: ctx.accounts.memecoin_config_wrapped_sol_account.to_account_info(),
            },
            &signer
        )
    )?;

    Ok(())
}