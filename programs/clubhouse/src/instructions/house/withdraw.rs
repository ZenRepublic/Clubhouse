
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{execute_token_transfer, House};


pub fn withdraw_house_fees(ctx: Context<WithdrawHouseFees>) -> Result<()> {
    let house = &mut ctx.accounts.house;
    let vault = &ctx.accounts.house_currency_vault;
    if vault.amount > 0 {
        execute_token_transfer(vault.amount, 
            ctx.accounts.house_currency_vault.to_account_info(), 
            ctx.accounts.admin_withdraw_account.to_account_info(), 
            house.to_account_info(), 
            ctx.accounts.token_program.to_account_info(), 
        Some(&[&[b"house",&house.house_name.as_bytes()[..], &[house.bump][..]]]))?;
    }

    let house_rent = Rent::minimum_balance(&Rent::get().unwrap(), 500);
    let current_lamports = ctx.accounts.house.get_lamports();
    let diff = current_lamports - house_rent;
    if diff > 0 {
        ctx.accounts.house.sub_lamports(diff)?;
        ctx.accounts.house_admin.add_lamports(diff)?;
    }
    ctx.accounts.house.unclaimed_sol_fees = 0;
    ctx.accounts.house.unclaimed_house_fees = 0;
    Ok(())
}


#[derive(Accounts)]
pub struct WithdrawHouseFees<'info> {
    #[account(mut,has_one=house_admin, has_one=house_currency)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub house_admin: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()], 
        bump,
        token::token_program = token_program
    )]
    pub house_currency_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed, payer=house_admin,
        associated_token::authority = house_admin,
        associated_token::mint = house_currency,
    )]
    pub admin_withdraw_account: InterfaceAccount<'info, TokenAccount>,

    pub house_currency: InterfaceAccount<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}