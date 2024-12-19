use anchor_lang::prelude::*;
use crate::state::GlobalConfig;

#[derive(Accounts)]
pub struct SetLaunchSuccessFeeBps<'info> {
    #[account(
        mut,
        has_one = admin,
        seeds = [b"CONFIG"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<SetLaunchSuccessFeeBps>,
    launch_success_fee_bps: u16,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    global_config.set_launch_success_fee_bps(launch_success_fee_bps)?;

    Ok(())
}
