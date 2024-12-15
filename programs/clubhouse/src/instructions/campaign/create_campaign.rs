
use anchor_lang::prelude::*;
use anchor_spl::{token_interface::TokenInterface, token_interface::{Mint, TokenAccount}};

use crate::{errors::{self, ErrorCodes}, execute_token_transfer, string_len_borsh, string_option_len, validate_string, Campaign, House, NftCampaignConfig, TimeSpan, TokenCampaignConfig, TokenUse};


pub fn create_campaign(ctx: Context<CreateCampaign>,
    campaign_name: String,
    uri: Option<String>,
    fund_amount: u64, 
    max_rewards_per_game: u64, 
    player_claim_price: u64, 
    time_span: TimeSpan, 
    nft_campaign_config: Option<NftCampaignConfig>, 
    token_campaign_config: Option<TokenCampaignConfig>) -> Result<()> {
    let campaign = &mut ctx.accounts.campaign;
    validate_string(&campaign_name)?;
    let clock = Clock::get()?;
    let ts_now = clock.unix_timestamp;

    if time_span.is_expired(ts_now) {
        return err!(errors::ErrorCodes::CampaignExpired);
    }

    if !time_span.is_valid() {
        return err!(errors::ErrorCodes::InvalidTimeSpan);
    }

    match token_campaign_config {
        Some(token_config) => {
            
            match (token_config.token_use, &ctx.accounts.game_mint, &ctx.accounts.game_deposit_vault) {
                (TokenUse::Burn, Some(_), None) => {},
                (_, Some(_), Some(_)) => {},
                (_, _, _) => return err!(ErrorCodes::InvalidInput),
            }
            if token_config.spending_mint != ctx.accounts.game_mint.as_ref().unwrap().key() {
                return err!(ErrorCodes::InvalidInput)
            }
            if token_config.spending_mint_decimals != ctx.accounts.game_mint.as_ref().unwrap().decimals {
                return err!(ErrorCodes::InvalidInput)
            }
        },
        None => {
        }
        
    };

    campaign.set_inner(Campaign {
        auth_bump: ctx.bumps.campaign_auth,
        reward_mint: ctx.accounts.reward_mint.key(),
        reward_mint_decimals: ctx.accounts.reward_mint.decimals,
        house: ctx.accounts.house.key(),
        campaign_name: campaign_name,
        uri: uri,
        house_config_snapshot: ctx.accounts.house.config.clone(),
        nft_config: nft_campaign_config,
        token_config: token_campaign_config,
        time_span: time_span,
        creator: ctx.accounts.signer.key(),
        max_rewards_per_game: max_rewards_per_game,
        rewards_claim_fee: player_claim_price,
        rewards_available: fund_amount,
        manager_mint: None,
        player_count: 0,
        active_games: 0,
        total_games: 0,
        unclaimed_sol_fees: 0,
        _reserved_config: [0; 7],
        _reserved_for_token: [0; 3],
        reserved_rewards: 0,
    });

    ctx.accounts.house.add_campaign();
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
#[instruction(
    campaign_name: String, 
    uri: Option<String>,)]
pub struct CreateCampaign<'info> {
    // for now, the campaign creator must be the house admin
    #[account(mut, constraint = signer.key() == house.house_admin)]
    pub signer: Signer<'info>,

    #[account(init, payer=signer, space=8+472+string_len_borsh(&campaign_name)+string_option_len(&uri))]
    pub campaign: Account<'info, Campaign>,

    /// CHECK: campaign proxy signer
    #[account(seeds=[campaign.key().as_ref()], bump)]
    pub campaign_auth: AccountInfo<'info>,

    #[account(mut)]
    pub house: Box<Account<'info, House>>,

    /// pays the campaign creation fees
    #[account(mut)]
    pub creation_fee_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub reward_mint: Box<InterfaceAccount<'info, Mint>>,

    /// the vault where we pay the campaign creation fees
    #[account(mut,seeds=[b"vault",house.key().as_ref()], bump)]
    pub house_vault: Box<InterfaceAccount<'info, TokenAccount>>,


    /// the account that deposits rewards for the campaign
    #[account(mut)]
    pub reward_depositor_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// the vault where the rewards are held to be claimed

    #[account(
        init, 
        payer=signer,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump, 
        token::mint = reward_mint, 
        token::authority = campaign_auth,
    )]
    pub reward_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    
    pub game_mint: Option<Box<InterfaceAccount<'info, Mint>>>,
    #[account(
        init, 
        payer=signer,
        seeds=[b"player_deposit", campaign.key().as_ref()], 
        bump, 
        token::mint = game_mint, 
        token::authority = campaign_auth,
    )]
    pub game_deposit_vault: Option<Box<InterfaceAccount<'info, TokenAccount>>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}