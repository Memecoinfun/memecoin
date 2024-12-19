use anchor_lang::prelude::*;
use crate::state::GlobalConfig;

#[derive(Accounts)]
pub struct SetCreateMemecoinFeeReceiver<'info> {
    #[account(
        mut,
        has_one = admin,
        seeds = [b"CONFIG"],
        bump
    )]
    pub global_config: Account<'info, GlobalConfig>,
}

pub fn handler(
    ctx: Context<SetCreateMemecoinFeeReceiver>,
    create_memecoin_fee_receiver: &Pubkey,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global_config;
    global_config.set_create_memecoin_fee_receiver(create_memecoin_fee_receiver);

    Ok(())
}
