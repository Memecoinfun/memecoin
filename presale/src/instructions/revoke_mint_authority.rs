use anchor_lang::prelude::*;

use anchor_spl::{
    token::{Token, Mint, SetAuthority, set_authority}
};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use crate::state::*;
use crate::errors::ErrorCode;



#[derive(Accounts)]
pub struct RevokeMintAuthority<'info> {
    #[account(
    mut,
    seeds = [memecoin_config.creator.key().as_ref(), &memecoin_config.creator_memecoin_index.to_le_bytes()],
    bump
    )]
    pub memecoin_config: Box<Account<'info, MemecoinConfig>>,

    #[account(
    mut,
    has_one = admin,
    seeds = [b"CONFIG"],
    bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
    mut,
    seeds = [b"mint", memecoin_config.key().as_ref()],
    mint::authority = memecoin_config,
    mint::decimals = 6,
    bump
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RevokeMintAuthority>) -> Result<()> {

    let cpi_accounts = SetAuthority {
        account_or_mint: ctx.accounts.mint.to_account_info(),
        current_authority: ctx.accounts.memecoin_config.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        &signer,
    );

    set_authority(cpi_context, AuthorityType::MintTokens, None)?;

    msg!("revoke mint_authority for mint token {}",ctx.accounts.mint.key().to_string());

    Ok(())
}
