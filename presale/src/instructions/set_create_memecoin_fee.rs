use anchor_lang::prelude::*;
use crate::state::GlobalConfig;

#[derive(Accounts)]
pub struct SetCreateMemecoinFee<'info> {
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
    ctx: Context<SetCreateMemecoinFee>,
    create_memecoin_fee: u64,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    global_config.set_create_memecoin_fee(create_memecoin_fee)?;

    Ok(())
}
