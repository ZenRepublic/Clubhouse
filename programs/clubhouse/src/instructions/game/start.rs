use std::ops::AddAssign;

use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token::{Token, TokenAccount}};
use crate::{campaign, errors, metadata_contains};

use crate::{errors::ErrorCodes, Campaign, CampaignPlayer, House};

pub fn start_game_with_nft(ctx: Context<StartGameWithNft>) -> Result<()> {
    let campaign_player = &mut ctx.accounts.campaign_player;
    require!(!campaign_player.in_game, ErrorCodes::PlayerInGame);
    if campaign_player.mint != ctx.accounts.player_nft_metadata.mint || campaign_player.campaign_slot != ctx.accounts.campaign.slot_created {
        //new or reinitialized campaign player
        campaign_player.set_inner(CampaignPlayer::new(&ctx.accounts.player_nft_metadata.mint, &ctx.accounts.campaign)?);
        ctx.accounts.house.unique_players.add_assign(1);
        ctx.accounts.campaign.player_count.add_assign(1);
    }
    let now_ts = Clock::get()?.unix_timestamp;
    if ctx.accounts.campaign.time_span.is_expired(now_ts) {
        return err!(errors::ErrorCodes::CampaignExpired);
    }
    let current_energy = campaign_player.recharge_energy(&ctx.accounts.campaign.nft_config, now_ts)?;
    msg!("current energy recharged to {}", current_energy);
    campaign_player.spend_energy(1);
    
    campaign_player.game_start_time = now_ts;

    ctx.accounts.campaign.active_games.add_assign(1);
    campaign_player.in_game = true;
    Ok(())
}

#[derive(Accounts)]
pub struct StartGameWithNft<'info> {
    #[account(mut)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub campaign: Box<Account<'info, Campaign>>,
    

    #[account(init_if_needed, space=144, seeds = [b"player", campaign.key().as_ref(), player_nft_metadata.mint.as_ref()], bump, payer = user)]
    pub campaign_player: Account<'info, CampaignPlayer>,

    #[account(
        constraint = player_nft_token_account.mint == player_nft_metadata.mint, 
        constraint = player_nft_token_account.owner == user.key() @ ErrorCodes::TokenOwnerMismatch, 
        constraint = player_nft_token_account.amount == 1 @ ErrorCodes::OwnerBalanceMismatch
    )]
    pub player_nft_token_account: Box<Account<'info, TokenAccount>>,


    #[account(constraint = metadata_contains(&player_nft_metadata,&campaign.nft_config.unwrap().collection))]
    pub player_nft_metadata: Box<Account<'info, MetadataAccount>>,


    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}