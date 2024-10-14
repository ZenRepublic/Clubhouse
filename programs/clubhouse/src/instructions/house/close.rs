use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{execute_token_close, execute_token_transfer, state::House};


pub fn close_house(ctx: Context<crate::CloseHouse>) -> Result<()> {
    let house = &mut ctx.accounts.house;
    if house.open_campaigns > 0 {
        return Err(crate::errors::ErrorCodes::ActiveCampaigns.into());
    }

    house.teardown();

    let vault = &ctx.accounts.house_currency_vault;
    if vault.amount > 0 {
        execute_token_transfer(vault.amount, 
            ctx.accounts.house_currency_vault.to_account_info(), 
            ctx.accounts.admin_withdraw_account.to_account_info(), 
            house.to_account_info(), 
            ctx.accounts.token_program.to_account_info(), 
        Some(&[&[b"house",&house.house_name.as_bytes()[..], &[house.bump][..]]]))?;
    }

    execute_token_close(vault.to_account_info(), 
    ctx.accounts.admin_withdraw_account.to_account_info(), 
    house.to_account_info(),
    ctx.accounts.token_program.to_account_info(), 
    Some(&[&[b"house",&house.house_name.as_bytes()[..], &[house.bump][..]]]))?;
    Ok(())
}

#[derive(Accounts)]
pub struct CloseHouse<'info> {
    #[account(mut, close=house_admin, has_one=house_admin, has_one=house_currency)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub house_admin: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()], 
        bump,
        token::token_program = token_program
    )]
    pub house_currency_vault: Account<'info, TokenAccount>,

    #[account(init_if_needed, payer=house_admin,
        associated_token::authority = house_admin,
        associated_token::mint = house_currency,
    )]
    pub admin_withdraw_account: Account<'info, TokenAccount>,

    pub house_currency: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
