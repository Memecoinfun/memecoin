use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;
pub mod amm;

declare_id!("8VmiQfMyGSeksAkHLuXYhpXccsqhkPavH26g1BTFjpmg");

#[program]
pub mod curve_launchpad {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::initialize(ctx)
    }

    pub fn init_invite_account(ctx:Context<InitInviteAccount>,parent: Pubkey) ->Result<()>{
        user_invite::init_invite(ctx, parent)
    }
    pub fn claim_invite_profit(ctx:Context<InviteClaimAccount>)->Result<()>{
        user_invite::claim_inivte_profit(ctx)
    }


    pub fn init_create_account(ctx: Context<InitCreateAccount>,seed:u64)->Result<()>{
        create::init_create_account(ctx, seed)
    }

    pub fn create(ctx: Context<Create>,
        name: String,
        symbol: String,
        uri: String,
        description: String,
        website: String,
        telegram: String,
        twitter: String,) -> Result<()> {
        create::create(ctx, name, symbol, uri,description,website,telegram,twitter)
    }

    pub fn buy(ctx: Context<Buy>, token_amount: u64, max_sol_cost: u64 , hash : String) -> Result<()> {
        buy::buy(ctx, token_amount, max_sol_cost,&hash)
    }

    pub fn sell(ctx: Context<Sell>, token_amount: u64, min_sol_output: u64 , hash : String) -> Result<()> {
        sell::sell(ctx, token_amount, min_sol_output,&hash)
    }

    pub fn protocol_fee_collect(ctx: Context<ProtocolFeeCollect>) -> Result<()>{
        withdraw::protocol_fee_collect(ctx)
    }

    pub fn wsol_sync_native(ctx: Context<WsolSyncNative>) ->Result<()>{
        withdraw::wsol_sync_native(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::withdraw(ctx)
    }

    pub fn set_amm_params(
        ctx: Context<SetParams>,
        initial_virtual_token_reserves: u64,
        initial_virtual_sol_reserves: u64,
        initial_real_token_reserves: u64,
        inital_token_supply: u64,
    ) -> Result<()> {
        set_params::set_amm_params(
            ctx,
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            inital_token_supply,
        )
    }

    pub fn set_fee_params(
        ctx: Context<SetParams>,
        fee_recipient: Pubkey,
        fee_basis_points: u64,
        creator_fee_basis_points: u64,
        withdraw_authority: Pubkey,
        invite_fee_basis_points:u64,
    ) -> Result<()> {
        set_params::set_fee_params(
            ctx,
            fee_recipient,
            fee_basis_points,
            creator_fee_basis_points,
            withdraw_authority,
            invite_fee_basis_points,
        )
    }

    pub fn set_protocol_fee_address(
        ctx: Context<SetParams>,
        protocol_token_alloc_recipient: Pubkey,
        fee_recipient: Pubkey
    ) -> Result<()> {
        set_params::set_protocol_fee_address(
            ctx,
            protocol_token_alloc_recipient,
            fee_recipient
        )
    }


}
