use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors, execute_token_close, execute_token_transfer, Campaign, House};

pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
    if false &&ctx.accounts.campaign.time_span.is_active(Clock::get()?.unix_timestamp) {
        return err!(errors::ErrorCodes::ActiveCampaign);
    }
    ctx.accounts.house.open_campaigns = ctx.accounts.house.open_campaigns.saturating_sub(1);
    
    execute_token_transfer(
        ctx.accounts.reward_vault.amount,
        ctx.accounts.reward_vault.to_account_info(),
        ctx.accounts.reward_withdrawal_account.to_account_info(),
        ctx.accounts.campaign_auth.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        Some(&[&[ctx.accounts.campaign.key().as_ref(), &[ctx.accounts.campaign.auth_bump]]]))?;
    execute_token_close(
        ctx.accounts.reward_vault.to_account_info(),
        ctx.accounts.creator.to_account_info(),
        ctx.accounts.campaign_auth.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        Some(&[&[ctx.accounts.campaign.key().as_ref(), &[ctx.accounts.campaign.auth_bump]]]))?;
    Ok(())
}

#[derive(Accounts)]
pub struct CloseCampaign<'info> {
    #[account(mut, close=creator, has_one=creator, has_one=house)]
    pub campaign: Box<Account<'info, Campaign>>,
    /// CHECK: the campaign auth PDA
    #[account()]
    pub campaign_auth: AccountInfo<'info>,
    /// the account that deposits rewards for the campaign
    #[account(init_if_needed, 
        payer=creator, 
        associated_token::authority=creator, 
        associated_token::mint = reward_mint, 
        associated_token::token_program = token_program)]
    pub reward_withdrawal_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// the vault where the rewards are held to be claimed

    pub reward_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump
    )]
    pub reward_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub creator: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}