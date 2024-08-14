use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token,
    token::{self, TokenAccount},
    token_interface,
};

use crate::{BuyWithSol, CreatePool, MULTIPLIER};

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

pub fn buy_with_sol_handle(ctx: Context<BuyWithSol>, sol_amount: u64) -> Result<()> {
    // calculate amountOut
    let (cost, amount_out) = calculate_amount_mout(&ctx, sol_amount);
    // transfer SOL
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.pool.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx, cost)?;
    // transfer token from PDA to caller
    // let bonding_curve_signer_seeds: [&[&[u8]]; 1] = [&[&bonding_curve_seed(), binding.as_ref(), &[ctx.accounts.bonding_curve.bump]]];

    let pool_signer_seeds: &[&[&[u8]]] =
        &[&[b"pool".as_ref(), &ctx.accounts.sell_token.key().to_bytes()]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program_2022.to_account_info(),
        token::Transfer {
            from: ctx.accounts.pool_sell_token_ata.to_account_info(),
            to: ctx.accounts.buyer_sell_token_ata.to_account_info(),
            authority: ctx.accounts.pool.to_account_info(),
        },
        &pool_signer_seeds,
    );
    token::transfer(cpi_ctx, amount_out)?;
    Ok(())
}

fn calculate_amount_mout(ctx: &Context<BuyWithSol>, sol_amount: u64) -> (u64, u64) {
    let amount_out = sol_amount * ctx.accounts.pool.price_per_sol / MULTIPLIER;
    let remain_balance = ctx.accounts.pool_sell_token_ata.amount;
    if amount_out > remain_balance {
        // buy only remain balance
        let cost = remain_balance * MULTIPLIER / ctx.accounts.pool.price_per_sol;
        return (cost, remain_balance);
    } else {
        return (sol_amount, amount_out);
    }
}
