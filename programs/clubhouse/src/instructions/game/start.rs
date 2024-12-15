use std::ops::AddAssign;

use anchor_lang::prelude::*;
use anchor_spl::{metadata::MetadataAccount, token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{errors, execute_token_burn, execute_token_transfer, metadata_contains, IdentityType, PlayerIdentity, StakeInfo, TokenUse};

use crate::{errors::ErrorCodes, Campaign, CampaignPlayer, House};

pub fn start_game(ctx: Context<StartGame>) -> Result<()> {

    let inferred_identity = ctx.accounts.get_player_identity()?;
    let campaign = &mut ctx.accounts.campaign;
    let campaign_player = &mut ctx.accounts.campaign_player;

    require!(!campaign_player.in_game, ErrorCodes::PlayerInGame);

    if campaign_player.player_identity.identity_type == IdentityType::None {
        //new or reinitialized campaign player
        campaign_player.set_inner(CampaignPlayer::new(inferred_identity, &campaign)?);
        ctx.accounts.house.unique_players.add_assign(1);
        campaign.player_count.add_assign(1);
    }

    require!(campaign_player.player_identity == inferred_identity, ErrorCodes::PlayerIdentityMismatch);

    match (&campaign.nft_config, campaign.token_config, inferred_identity.identity_type, &ctx.accounts.player_nft_metadata) {
        (None, Some(token_config), IdentityType::User, None) => {
            let payment_amount = token_config.energy_price;
            match token_config.token_use {
                crate::TokenUse::Stake | crate::TokenUse::Pay => 
                {
                    execute_token_transfer(
                    payment_amount, 
                    ctx.accounts.players_deposit_account.as_ref().unwrap().to_account_info(),
                    ctx.accounts.game_deposit_vault.as_ref().unwrap().to_account_info(),
                     ctx.accounts.user.to_account_info(),
                      ctx.accounts.token_program.to_account_info(), 
                      None)?;
                    
                    if token_config.token_use == crate::TokenUse::Stake {
                        campaign_player.stake_info.as_mut().unwrap().amount.add_assign(payment_amount);
                    }
                }
                crate::TokenUse::Burn => {
                    execute_token_burn(
                        payment_amount, 
                        ctx.accounts.game_deposit_mint.as_ref().unwrap().to_account_info(), 
                        ctx.accounts.players_deposit_account.as_ref().unwrap().to_account_info(), 
                        ctx.accounts.user.to_account_info(),
                        ctx.accounts.token_program.to_account_info(), None
                    )?;
                }
            }
        },
        (Some(_), None, IdentityType::Nft, Some(_)) => {
            
        },

        (_, _, _ , _,) => return err!(ErrorCodes::InvalidInput),
    };

    match (&ctx.accounts.player_nft_token_account, &ctx.accounts.player_nft_metadata, &ctx.accounts.game_deposit_mint, &ctx.accounts.game_deposit_vault, &ctx.accounts.players_deposit_account) {
        (Some(_), Some(_), Some(_), Some(_), Some(_)) => {},
        (Some(_), Some(_), None, None, None) => {},
        (None, None, Some(_), None, Some(_)) => {},
        (None, None, Some(_), Some(_), Some(_)) => {},
        (_, _, _, _, _) => return err!(ErrorCodes::InvalidInput),
    };


    let max_rewards = campaign.max_rewards_per_game;
 
    let now_ts = Clock::get()?.unix_timestamp;
    if campaign.time_span.is_expired(now_ts) {
        return err!(errors::ErrorCodes::CampaignExpired);
    }
    let _ = campaign_player.recharge_energy(&campaign.nft_config, now_ts)?;
    
    campaign_player.spend_energy(1)?;
    
    campaign_player.game_start_time = now_ts;

    campaign.active_games.add_assign(1);
    campaign.reserved_rewards.add_assign(max_rewards);
    if campaign.rewards_available < campaign.reserved_rewards{
       return  err!(ErrorCodes::RewardsUnavailable)
    }
    
    campaign_player.in_game = true;
    Ok(())
}

#[derive(Accounts)]
pub struct StartGame<'info> {
    #[account(mut)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub campaign: Box<Account<'info, Campaign>>,
    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    #[account(init_if_needed, space=8+CampaignPlayer::INIT_SPACE - campaign.token_config.map_or(StakeInfo::INIT_SPACE, |t| if t.token_use == TokenUse::Stake {0} else {StakeInfo::INIT_SPACE}), seeds = [b"player", campaign.key().as_ref(), &player_nft_metadata.as_ref().map_or(user.key().to_bytes(), |m| m.mint.to_bytes())[..]], bump, payer = user)]
    pub campaign_player: Account<'info, CampaignPlayer>,

    #[account(
        constraint = player_nft_metadata.as_ref().is_some_and(|m| m.mint == player_nft_token_account.mint ), 
        constraint = player_nft_token_account.owner == user.key() @ ErrorCodes::TokenOwnerMismatch, 
        constraint = player_nft_token_account.amount == 1 @ ErrorCodes::OwnerBalanceMismatch,
    )]
    pub player_nft_token_account: Option<Box<InterfaceAccount<'info, TokenAccount>>>,


    #[account(constraint = metadata_contains(&player_nft_metadata,&campaign.nft_config.unwrap().collection).is_ok())]
    pub player_nft_metadata: Option<Box<Account<'info, MetadataAccount>>>,

    #[account(mut)]
    pub game_deposit_mint: Option<Box<InterfaceAccount<'info, Mint>>>,
    #[account(mut)]
    pub players_deposit_account: Option<Box<InterfaceAccount<'info, TokenAccount>>>,

    #[account(mut, 
        seeds=[b"player_deposit", campaign.key().as_ref()], bump
    )]
    pub game_deposit_vault: Option<Box<InterfaceAccount<'info, TokenAccount>>>,
}


impl StartGame<'_> {
    pub fn get_player_identity(&self) -> Result<PlayerIdentity> {
        match (self.campaign.nft_config, &self.player_nft_metadata) {
            (None, None) => Ok(PlayerIdentity{
                    identity_type: IdentityType::User,
                    pubkey: self.user.key(),
                }),
            (None, Some(_)) => err!(ErrorCodes::UnexpectedMetadata),
            (Some(_), None) => err!(ErrorCodes::MissingMetadata),
            (Some(_), Some(_)) => Ok(PlayerIdentity{
             identity_type: IdentityType::Nft,
             pubkey: self.player_nft_metadata.as_ref().unwrap().mint
            }),
        }
    }
}

