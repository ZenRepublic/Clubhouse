use anchor_lang::prelude::*;
use anchor_spl::{token_interface::TokenInterface, token_interface::{Mint, TokenAccount}, metadata::MetadataAccount};

use crate::{errors::{self, ErrorCodes}, execute_token_transfer, metadata_is_collection, state::ManagerSlot, string_len_borsh, string_option_len, validate_string, Campaign, House, NftCampaignConfig, TimeSpan, TokenCampaignConfig, TokenUse};


pub fn create_campaign(ctx: Context<CreateCampaign>,
    campaign_name: String,
    uri: Option<String>,
    fund_amount: u64, 
    max_rewards_per_game: u64, 
    player_claim_price: u64, 
    time_span: TimeSpan, 
    nft_campaign_config: Option<NftCampaignConfig>, 
    token_campaign_config: Option<TokenCampaignConfig>) -> Result<()> {
    validate_string(&campaign_name)?;
    let clock = Clock::get()?;
    let ts_now = clock.unix_timestamp;


    if time_span.is_expired(ts_now) {
        return err!(errors::ErrorCodes::CampaignExpired);
    }

    if !time_span.is_valid() {
        return err!(errors::ErrorCodes::InvalidTimeSpan);
    }

    // Validate manager NFT if provided
    let mut signer_must_pay = true;
    if ctx.accounts.signer.key() != ctx.accounts.house.house_admin {
    if ctx.accounts.house.manager_collection.is_some() {
            require!(ctx.accounts.manager_nft_token_account.is_some(), ErrorCodes::MetadataMismatch);
            require!(ctx.accounts.manager_nft_metadata.is_some(), ErrorCodes::MetadataMismatch);
            
            let token_account = ctx.accounts.manager_nft_token_account.as_ref().unwrap();
            require!(token_account.owner == ctx.accounts.signer.key(), ErrorCodes::TokenOwnerMismatch);
            require!(token_account.amount == 1, ErrorCodes::OwnerBalanceMismatch);
            
            let metadata = ctx.accounts.manager_nft_metadata.as_ref().unwrap();
            require!(metadata_is_collection(metadata, &ctx.accounts.house.manager_collection.unwrap()).is_ok(), ErrorCodes::CollectionProofInvalid);
            require!(ctx.accounts.manager_slot.is_some(), ErrorCodes::InvalidInput);
            signer_must_pay = false;
            ctx.accounts.manager_slot.as_mut().unwrap().manager = ctx.accounts.manager_nft_metadata.as_ref().unwrap().mint;
            ctx.accounts.manager_slot.as_mut().unwrap().campaign = ctx.accounts.campaign.key();


        }
        else {
            require!(ctx.accounts.manager_nft_token_account.is_none(), ErrorCodes::InvalidInput);
            require!(ctx.accounts.manager_nft_metadata.is_none(), ErrorCodes::InvalidInput);
            require!(ctx.accounts.manager_slot.is_none(), ErrorCodes::InvalidInput);
            require!(ctx.accounts.creation_fee_account.is_some(), ErrorCodes::InvalidInput);
        }
    }
    else {
        signer_must_pay = false;
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

    let campaign = &mut ctx.accounts.campaign;
    campaign.auth_bump = ctx.bumps.campaign_auth;
    campaign.reward_mint = ctx.accounts.reward_mint.key();
    campaign.reward_mint_decimals = ctx.accounts.reward_mint.decimals;
    campaign.house = ctx.accounts.house.key();
    campaign.campaign_name = campaign_name;
    campaign.uri = uri;
    campaign.house_config_snapshot = ctx.accounts.house.config.clone();
    campaign.nft_config = nft_campaign_config;
    campaign.token_config = token_campaign_config;
    campaign.time_span = time_span;
    campaign.creator = ctx.accounts.signer.key();
    campaign.max_rewards_per_game = max_rewards_per_game;
    campaign.rewards_claim_fee = player_claim_price;
    campaign.rewards_available = fund_amount;
    campaign.manager_mint = None;
    campaign.player_count = 0;
    campaign.active_games = 0;
    campaign.total_games = 0;
    campaign.unclaimed_sol_fees = 0;
    campaign._reserved_config = [0; 7];
    campaign._reserved_for_token = [0; 3];
    campaign.reserved_rewards = 0;

    ctx.accounts.house.add_campaign();
    let fee = match signer_must_pay {
        true => ctx.accounts.house.config.campaign_creation_fee,
        false => 0
    };
    
    ctx.accounts.house.unclaimed_house_fees += fee;
    if fee > 0 {
        execute_token_transfer(
            fee, 
            ctx.accounts.creation_fee_account.as_ref().unwrap().to_account_info(), 
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
    #[account(mut)]
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
    pub creation_fee_account: Option<Box<InterfaceAccount<'info, TokenAccount>>>,

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

    /// Optional manager NFT token account
    pub manager_nft_token_account: Option<Box<InterfaceAccount<'info, TokenAccount>>>,

    /// Optional manager NFT metadata
    pub manager_nft_metadata: Option<Box<Account<'info, MetadataAccount>>>,

    #[account(
        init,
        space= 8+32+32,
        payer=signer,
        seeds=[b"manager_slot", manager_nft_metadata.as_ref().unwrap().mint.key().as_ref()], bump,
        constraint = house.manager_collection.is_some() @ ErrorCodes::InvalidInput,
        constraint = metadata_is_collection(&manager_nft_metadata.as_ref().unwrap(),&house.manager_collection.unwrap()).is_ok() @ ErrorCodes::CollectionProofInvalid
        
    )]
    pub manager_slot: Option<Account<'info,ManagerSlot>>,
}