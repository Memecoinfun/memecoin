use crate::{
    state::{BondingCurve, Global, UserInviteStats}, CreateEvent, CurveLaunchpadError, DEFAULT_DECIMALS
};
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{
        self, mint_to, spl_token::instruction::AuthorityType, Mint, MintTo, Token, TokenAccount,
    },
};


#[derive(Accounts)]
#[instruction(_seed: u64)]
pub struct InitCreateAccount<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [ &_seed.to_le_bytes()],
        bump,
        mint::decimals = DEFAULT_DECIMALS as u8,
        mint::authority = bonding_curve,
    )]
    mint: Account<'info, Mint>,


    #[account(
        init,
        payer = creator,
        seeds = [BondingCurve::SEED_PREFIX, mint.to_account_info().key.as_ref()],
        bump,
        space = 8 + BondingCurve::INIT_SPACE,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,

}

pub fn  init_create_account(_ctx: Context<InitCreateAccount>,
    _seed: u64,
) -> Result<()> {

    Ok(())
}


#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        mut,
        mint::decimals = DEFAULT_DECIMALS as u8,
        mint::authority = bonding_curve,
    )]
    mint: Account<'info, Mint>,

    #[account(mut)]
    creator: Signer<'info>,


    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX, mint.to_account_info().key.as_ref()],
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = bonding_curve,
        seeds=[BondingCurve::SEED_PREFIX, mint.key().as_ref(), bonding_curve.key().as_ref()],
        bump
    )]
    bonding_curve_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [Global::SEED_PREFIX],
        bump,
    )]
    global: Box<Account<'info, Global>>,

    ///CHECK: Using seed to validate metadata account
    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key.as_ref(), 
            mint.to_account_info().key.as_ref()
        ],
        seeds::program = token_metadata_program.key(),
        bump,
    )]
    metadata: AccountInfo<'info>,


    #[account(
        mut,
        seeds=[UserInviteStats::SEED_PREFIX,creator.key().as_ref()],
        bump
    )]

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    associated_token_program: Program<'info, AssociatedToken>,

    token_metadata_program: Program<'info, Metaplex>,

    rent: Sysvar<'info, Rent>,
    clock: Sysvar<'info, Clock>,

}


pub fn create(ctx: Context<Create>,
    name: String,
    symbol: String,
    uri: String,
    description: String,
    website: String,
    telegram: String,
    twitter: String,
) -> Result<()> {
    //confirm program is initialized
    require!(
        ctx.accounts.global.initialized,
        CurveLaunchpadError::NotInitialized
    );


    // Verify that the mint address ends with "meme"
    require!(
        ctx.accounts.mint.key().to_string().to_lowercase().ends_with("meme"),
        CurveLaunchpadError::InvalidMintAddress
    );

    let signer: [&[&[u8]]; 1] = [&[
        BondingCurve::SEED_PREFIX,
        ctx.accounts.mint.to_account_info().key.as_ref(),
        &[ctx.bumps.bonding_curve],
    ]];

    let token_data: DataV2 = DataV2 {
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let metadata_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            payer: ctx.accounts.creator.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            metadata: ctx.accounts.metadata.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
        &signer,
    );

    create_metadata_accounts_v3(metadata_ctx, token_data, false, true, None)?;

    //mint tokens to bonding_curve_token_account
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.bonding_curve.to_account_info(),
                to: ctx.accounts.bonding_curve_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            },
            &signer,
        ),
        ctx.accounts.global.initial_token_supply,
    )?;

    //remove mint_authority
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::SetAuthority {
            current_authority: ctx.accounts.bonding_curve.to_account_info(),
            account_or_mint: ctx.accounts.mint.to_account_info(),
        },
        &signer,
    );
    token::set_authority(cpi_context, AuthorityType::MintTokens, None)?;

    let current_timestamp = ctx.accounts.clock.unix_timestamp as u64;


    let bonding_curve = &mut ctx.accounts.bonding_curve;
    bonding_curve.virtual_token_reserves = ctx.accounts.global.initial_virtual_token_reserves;
    bonding_curve.real_token_reserves = ctx.accounts.global.initial_real_token_reserves;
    bonding_curve.token_total_supply = ctx.accounts.global.initial_token_supply;
    bonding_curve.complete = false;

    bonding_curve.pool_sol_amount = 0;
    bonding_curve.pool_token_amount = 0;
    bonding_curve.creator = ctx.accounts.creator.to_account_info().key();
    bonding_curve.mint = ctx.accounts.mint.to_account_info().key();
    bonding_curve.create_time = current_timestamp;


    let create_event = CreateEvent {
        name:name.clone(),
        symbol:symbol.clone(),
        uri:uri.clone(),
        mint: ctx.accounts.mint.to_account_info().key().to_string(),
        creator: ctx.accounts.creator.to_account_info().key().to_string(),
        created_time: current_timestamp,
        destination: ctx.accounts.bonding_curve_token_account.to_account_info().key().to_string(),
        description: description.clone(),
        website: website.clone(),
        telegram: telegram.clone(),
        twitter: twitter.clone(),
        decimal: 6,
    };

    //emit_cpi!(createEvent);

    let serialized = serde_json::to_string(&create_event).unwrap();

    msg!("MemecoinCreated:{}", serialized);


    Ok(())
}
