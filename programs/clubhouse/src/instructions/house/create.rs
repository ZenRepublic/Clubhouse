use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{House, HouseConfig};


pub fn create_house(ctx: Context<CreateHouse>, manager_collection: Option<Pubkey>, house_config: HouseConfig, house_name: String) -> Result<()> {
    
    ctx.accounts.house.initialize(
        ctx.accounts.house_admin.key(),
        manager_collection,
        ctx.accounts.house_currency_mint.key(),
        ctx.accounts.house_currency_mint.decimals,
        house_config,
        house_name,
        ctx.bumps.house
    )?;
    Ok(())
}


#[derive(Accounts)]
#[instruction(manager_collection: Option<Pubkey>, house_config: HouseConfig, house_name: String)]

pub struct CreateHouse<'info> {

    #[account(mut)]
    pub program_admin: Signer<'info>,
    #[account(seeds=[b"program_admin", program_admin.key().as_ref()], bump, has_one=program_admin)]
    pub program_admin_proof: Account<'info, crate::state::ProgramAdminProof>,
    #[account(init, payer=program_admin, space=8+492, seeds=[b"house", house_name.as_bytes().as_ref()], bump)]
    pub house: Box<Account<'info, House>>,
    /// CHECK: signing pda for house
    #[account(mut, seeds=[house.key().as_ref()], bump)]
    pub house_auth: AccountInfo<'info>,
    #[account(
        init, 
        payer=program_admin, 
        seeds=[b"vault", house.key().as_ref()], 
        bump, 
        token::mint = house_currency_mint, 
        token::authority = house,
        token::token_program = token_program
    )]
    pub house_currency_vault: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: House admin can be any account
    pub house_admin: AccountInfo<'info>,
    #[account(
        mint::token_program = token_program,
    )]
    pub house_currency_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
