use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::MetadataAccount, token::{Mint, Token, TokenAccount}};
use crate::{campaign, execute_lamport_transfer, execute_token_transfer, metadata_contains};

use crate::{errors::ErrorCodes, Campaign, CampaignPlayer, House};

pub fn end_game_with_nft(ctx: Context<EndGameWithNft>, amount_won: u64) -> Result<()> {
    let campaign_player = &mut ctx.accounts.campaign_player;
    if campaign_player.mint != ctx.accounts.player_nft_metadata.mint {
        //new
        return err!(ErrorCodes::TokenOwnerMismatch)

    }
    if campaign_player.campaign != ctx.accounts.campaign.key() {
        return err!(ErrorCodes::CollectionProofInvalid)
    }
   match &ctx.accounts.oracle {
        Some(oracle) => {
            if ctx.accounts.campaign.house_config_snapshot.oracle_key != oracle.key() {
                return err!(ErrorCodes::OracleMismatch)
            }
        }
        None => {
            if ctx.accounts.campaign.house_config_snapshot.oracle_key != System::id() {
                return err!(ErrorCodes::OracleMismatch)
            }
        }
   } 
   let pre_balance = ctx.accounts.reward_vault.amount;
   if amount_won > 0 {
        if amount_won > ctx.accounts.campaign.max_rewards_per_game {return err!(ErrorCodes::AmountTooHigh)}
   
        execute_token_transfer(amount_won, 
            ctx.accounts.reward_vault.to_account_info(), 
            ctx.accounts.player_reward_token_account.to_account_info(), 
            ctx.accounts.campaign.to_account_info(),
            ctx.accounts.token_program.to_account_info(), 
            Some(&[&[b"campaign",ctx.accounts.campaign.campaign_name.as_bytes().as_ref(), ctx.accounts.house.key().as_ref(),&[ctx.accounts.campaign.bump]]]))?;
            
        if ctx.accounts.campaign.house_config_snapshot.claim_fee > 0 {
            execute_lamport_transfer(
                ctx.accounts.campaign.house_config_snapshot.claim_fee,
                ctx.accounts.user.to_account_info(), 
                ctx.accounts.house.to_account_info(), 
                ctx.accounts.system_program.to_account_info(),
                None)?;
            ctx.accounts.house.unclaimed_sol_fees += ctx.accounts.campaign.house_config_snapshot.claim_fee;
        }
        if ctx.accounts.campaign.rewards_claim_fee > 0 {
            execute_lamport_transfer(ctx.accounts.campaign.rewards_claim_fee,
                ctx.accounts.user.to_account_info(), 
                ctx.accounts.campaign.to_account_info(), 
                ctx.accounts.system_program.to_account_info(),
                None)?;
            ctx.accounts.campaign.unclaimed_sol_fees += ctx.accounts.campaign.rewards_claim_fee;
        }
    }
    campaign_player.in_game = false;
    campaign_player.games_played += 1;
    ctx.accounts.campaign.active_games = ctx.accounts.campaign.active_games.saturating_sub(1);
    ctx.accounts.campaign.total_games +=1;
    ctx.accounts.campaign.rewards_available = pre_balance - amount_won;

    ctx.accounts.campaign.reserved_rewards -= ctx.accounts.campaign.max_rewards_per_game;
    if ctx.accounts.campaign.init_funding == 0 {
        ctx.accounts.campaign.init_funding = ctx.accounts.campaign.rewards_available;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct EndGameWithNft<'info> {
    #[account(mut)]
    pub house: Box<Account<'info, House>>,
    #[account(mut, has_one=house, has_one=reward_mint)]
    pub campaign: Box<Account<'info, Campaign>>,
    

    #[account(init_if_needed, space=144, seeds = [b"player", campaign.key().as_ref(), player_nft_metadata.mint.as_ref()], bump, payer = user)]
    pub campaign_player: Box<Account<'info, CampaignPlayer>>,

    #[account(
        constraint = player_nft_token_account.mint == player_nft_metadata.mint, 
        constraint = player_nft_token_account.owner == user.key() @ ErrorCodes::TokenOwnerMismatch, 
        constraint = player_nft_token_account.amount == 1 @ ErrorCodes::OwnerBalanceMismatch
    )]
    pub player_nft_token_account: Box<Account<'info, TokenAccount>>,


    #[account(constraint = metadata_contains(&player_nft_metadata,&campaign.nft_config.unwrap().collection))]
    pub player_nft_metadata: Box<Account<'info, MetadataAccount>>,

    #[account()]
    pub reward_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump, 
        token::mint = reward_mint, 
        token::authority = campaign
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,
    #[account(init_if_needed, payer=user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub player_reward_token_account : Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub oracle: Option<Signer<'info>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}