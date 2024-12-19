use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = GlobalConfig::LEN,
        seeds = [b"CONFIG"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeGlobalConfig>,
    create_memecoin_fee_receiver: Pubkey,
    launch_success_fee_receiver: Pubkey,
    create_memecoin_fee: u64,
    launch_success_fee_bps: u16,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    let admin_key = ctx.accounts.admin.key();
    global_config.initialize(
        admin_key,
        create_memecoin_fee_receiver,
        launch_success_fee_receiver,
        create_memecoin_fee,
        launch_success_fee_bps
    )?;

    Ok(())
}
