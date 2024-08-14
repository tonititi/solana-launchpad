use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token,
    token::{self, TokenAccount},
    token_interface,
};

use crate::CreatePool;

pub fn create_pool_handle(
    ctx: Context<CreatePool>,

    amount: u64,
    price_per_sol: u64,
    price_per_usdt: u64,
) -> Result<()> {
    // transfer token from caller to PDA
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program_2022.to_account_info(),
        token::Transfer {
            from: ctx.accounts.creator_sell_token_ata.to_account_info(),
            to: ctx.accounts.pool_sell_token_ata.to_account_info(),
            authority: ctx.accounts.creator.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;
    ctx.accounts.pool.sell_token = ctx.accounts.sell_token.key();
    ctx.accounts.pool.amount = amount;
    ctx.accounts.pool.price_per_sol = price_per_sol;
    ctx.accounts.pool.price_per_usdt = price_per_usdt;
    ctx.accounts.launchpad_config.total_pool += 1;
    Ok(())
}
