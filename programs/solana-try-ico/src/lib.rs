use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{Mint, Token2022, TokenAccount, TokenInterface}};


mod logics;
declare_id!("GRZnyMk3yAkvcTb3HbPjKFcdEGXBDAwfudVWZZ1RfjxU");

#[program]
pub mod solana_try_ico {
    use logics::{buy_with_sol_handle, create_pool_handle};

    use super::*;
    pub static MULTIPLIER: u64 = 1_000_000_000_000_u64;

    pub fn initialize(ctx: Context<Initialize>, treasury: Pubkey, launchpad_fee: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.launchpad_config.set_inner(LaunchpadConfig { owner: ctx.accounts.initializer.to_account_info().key(), launchpad_fee, treasury, bump: ctx.bumps.launchpad_config, total_pool: 0_u64 });
        Ok(())
    }
    pub fn create_pool(ctx: Context<CreatePool>,  amount: u64, price_per_sol: u64, price_per_usdt: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        create_pool_handle(ctx,  amount, price_per_sol, price_per_usdt)
    }
    pub fn buy_with_sol(ctx: Context<BuyWithSol>, sol_amount: u64)-> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        buy_with_sol_handle(ctx, sol_amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        space = 8 + 32+ 8 + 32 + 1,
        seeds = [b"launchpad_config".as_ref(), &1_u64.to_le_bytes()],
        bump 
    )]
    pub launchpad_config: Account<'info, LaunchpadConfig>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CreatePool<'info>{
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        payer = creator,
        space = 8 + 32 * 2  + 8 * 2 + 1,
        seeds = [b"pool".as_ref(), &sell_token.key().to_bytes()],
        bump
    )]
    pub pool: Account<'info, Pool>,
    /// CHECK: read only
    pub sell_token: InterfaceAccount<'info, Mint>,
    /// Create ATA
    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = sell_token,
        associated_token::authority = pool,
        associated_token::token_program = token_program_2022,
    )]
    pub pool_sell_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = sell_token,
        associated_token::authority = creator,
        associated_token::token_program = token_program_2022,
        
    )]
    pub creator_sell_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"launchpad_config".as_ref(), &1_u64.to_le_bytes()],
        bump = launchpad_config.bump 
    )]
    pub launchpad_config: Account<'info, LaunchpadConfig>,
    pub token_program_2022: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct LaunchpadConfig {
    pub owner: Pubkey,
    pub launchpad_fee: u64,
    pub treasury: Pubkey,
    pub bump: u8,
    pub total_pool: u64,
}
#[account]
pub struct Pool {
    pub owner: Pubkey,
    pub sell_token: Pubkey,
    pub amount: u64,
    pub price_per_sol: u64, // with MULTIPLIER
    pub price_per_usdt: u64, // with MULTIPLIER
}

#[derive(Accounts)]
pub struct BuyWithSol<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub pool_sell_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub creator_sell_token_ata: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: read only
    pub sell_token: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = sell_token,
        associated_token::authority = pool,
        associated_token::token_program = token_program_2022,
    )]
    pub buyer_sell_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub launchpad_config: Account<'info, LaunchpadConfig>,
    pub token_program_2022: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
