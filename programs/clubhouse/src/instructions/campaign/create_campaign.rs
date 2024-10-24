use std::ops::AddAssign;

use anchor_lang::prelude::*;
use anchor_spl::{token::Token, token::{Mint, TokenAccount}};

use crate::{errors::{self, ErrorCodes}, execute_token_transfer, validate_string, Campaign, House, NftCampaignConfig, TimeSpan, TokenCampaignConfig};


pub fn create_campaign(ctx: Context<CreateCampaign>,
    campaign_name: String, 
    fund_amount: u64, 
    max_rewards_per_game: u64, 
    player_claim_price: u64, 
    time_span: TimeSpan, 
    nft_energy_config: Option<NftCampaignConfig>, 
    token_energy_config: Option<TokenCampaignConfig>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    validate_string(&campaign_name)?;
    campaign.bump = ctx.bumps.campaign;
    campaign.reward_mint = ctx.accounts.reward_mint.key();
    campaign.reward_mint_decimals = ctx.accounts.reward_mint.decimals;
    campaign.house = ctx.accounts.house.key();
    campaign.campaign_name = campaign_name;
    campaign.house_config_snapshot = ctx.accounts.house.config.clone();
    campaign.nft_config = nft_energy_config;
    campaign.token_config = token_energy_config;
    campaign.house = ctx.accounts.house.key();
    campaign.time_span = time_span;
    campaign.creator = ctx.accounts.signer.key();
    campaign.max_rewards_per_game = max_rewards_per_game;
    campaign.rewards_claim_fee = player_claim_price;
    campaign.rewards_available = fund_amount;
    campaign.init_funding = fund_amount;
    let clock = Clock::get()?;
    let ts_now = clock.unix_timestamp;
    campaign.slot_created = clock.slot;
    if campaign.time_span.is_expired(ts_now) {
        return err!(errors::ErrorCodes::CampaignExpired);
    }

    

    ctx.accounts.house.total_campaigns.add_assign(1);

    let fee = ctx.accounts.house.config.campaign_creation_fee;
    ctx.accounts.house.unclaimed_house_fees += fee;
    if fee > ctx.accounts.creation_fee_account.amount { return err!(ErrorCodes::InsufficientFunds)}
    if fee > 0 {
        execute_token_transfer(
            fee, 
            ctx.accounts.creation_fee_account.to_account_info(), 
            ctx.accounts.house_vault.to_account_info(), 
            ctx.accounts.signer.to_account_info(), 
            ctx.accounts.token_program.to_account_info(),
            None)?;
    }

    if fund_amount > 0 {
        execute_token_transfer(
            fund_amount, 
            ctx.accounts.reward_depositor_account.to_account_info(), 
            ctx.accounts.reward_vault.to_account_info(), 
            ctx.accounts.signer.to_account_info(), 
            ctx.accounts.token_program.to_account_info(),
            None)?;
    }

    Ok(())
}


#[derive(Accounts)]
#[instruction(campaign_name: String)]
pub struct CreateCampaign<'info> {
    // for now, the campaign creator must be the house admin
    #[account(mut, constraint = signer.key() == house.house_admin)]
    pub signer: Signer<'info>,

    #[account(init, payer=signer, space=9+496+24+campaign_name.len(), seeds=[b"campaign", campaign_name.as_bytes(), house.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub house: Box<Account<'info, House>>,

    /// pays the campaign creation fees
    #[account(mut)]
    pub creation_fee_account: Box<Account<'info, TokenAccount>>,

    pub reward_mint: Box<Account<'info, Mint>>,
    /// the vault where we pay the campaign creation fees
    #[account(mut,seeds=[b"vault",house.key().as_ref()], bump)]
    pub house_vault: Box<Account<'info, TokenAccount>>,


    /// the account that deposits rewards for the campaign
    #[account(mut)]
    pub reward_depositor_account: Box<Account<'info, TokenAccount>>,
    /// the vault where the rewards are held to be claimed

    #[account(
        init, 
        payer=signer,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump, 
        token::mint = reward_mint, 
        token::authority = campaign,
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}