use anchor_lang::prelude::*;
use crate::state::*;
use anchor_lang::{
    solana_program::{
        clock::UnixTimestamp,
        sysvar::clock::Clock,
        system_instruction::transfer as lamports_transfer,
    }
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        CreateMetadataAccountsV3,
        Metadata,
        mpl_token_metadata::types::DataV2,
    },
    token::{Token, TokenAccount, mint_to, MintTo, Mint},
    //token_2022::{mint_to, MintTo},
    //token_interface::Mint,
};
use anchor_spl::token_interface::TokenInterface;
//use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount };
use mpl_token_metadata::pda::find_metadata_account;
use solana_program::program::invoke;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateMemecoinConfig<'info> {
    #[account(
    init_if_needed,
    payer = creator,
    space = CreatorMemecoinCounter::LEN,
    seeds = [b"COUNTER", creator.key().as_ref()],
    bump
    )]
    pub creator_memecoin_counter: Account<'info, CreatorMemecoinCounter>,

    #[account(
    init,
    payer = creator,
    space = MemecoinConfig::LEN,
    seeds = [creator.key().as_ref(), & creator_memecoin_counter.count.to_le_bytes()],
    bump
    )]
    pub memecoin_config: Account<'info, MemecoinConfig>,

    ///CHECK: Using "address" constraint to validate fee receiver address
    #[account(
    mut,
    address = global_config.create_memecoin_fee_receiver
    )]
    pub create_memecoin_fee_receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
    seeds = [b"CONFIG"],
    bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
}

pub fn handler(
    ctx: Context<CreateMemecoinConfig>,
    funding_raise_tier: u8,
) -> Result<()> {
    let creator = &ctx.accounts.creator.key();
    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;
    msg!("=======================create memecoinconfig.");


    if ctx.accounts.global_config.create_memecoin_fee > 0 {
        // Charge for the create memecoin fee
        let transfer_instruction = lamports_transfer(
            &ctx.accounts.creator.key(),
            &ctx.accounts.create_memecoin_fee_receiver.key(),
            ctx.accounts.global_config.create_memecoin_fee,
        );
        invoke(
            &transfer_instruction,
            &[
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.create_memecoin_fee_receiver.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }


    let memecoin_config = &mut ctx.accounts.memecoin_config;
    let tier = match funding_raise_tier {
        0 => FundingRaiseTier::TwentySol,
        1 => FundingRaiseTier::FiftySol,
    };
    memecoin_config.create_memecoin_config(
        creator,
        ctx.accounts.creator_memecoin_counter.count,
        current_timestamp,
        tier,
    )?;

    let creator_memecoin_counter = &mut ctx.accounts.creator_memecoin_counter;
    creator_memecoin_counter.increment();

    Ok(())
}
