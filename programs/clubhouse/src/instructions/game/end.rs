use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::MetadataAccount, token::{Mint, Token, TokenAccount}};
use crate::{execute_lamport_transfer, execute_token_transfer, metadata_contains, state::{SimplifiedAssetV1, UpdateAuthority}, IdentityType, PlayerIdentity};

use crate::{errors::ErrorCodes, Campaign, CampaignPlayer, House};

pub fn end_game(ctx: Context<EndGame>, amount_won: u64) -> Result<()> {
    ctx.accounts.validate_core_nft()?;
    let campaign_player = &mut ctx.accounts.campaign_player;
    match (ctx.accounts.campaign.nft_config, &ctx.accounts.player_nft_metadata, &ctx.accounts.player_nft_token_account) {
        (None, None, None) => {},
        (Some(_), Some(_), Some(_)) => {},
        (Some(_), _, _) => return err!(ErrorCodes::MissingMetadata),
        (None, _, _) => return err!(ErrorCodes::UnexpectedMetadata),
        
    }

    if campaign_player.campaign != ctx.accounts.campaign.key() {
        return err!(ErrorCodes::CollectionProofInvalid)
    }

    require!(campaign_player.in_game, ErrorCodes::RewardsUnavailable);
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
   
   if amount_won > 0 {
        if amount_won > ctx.accounts.campaign.max_rewards_per_game {return err!(ErrorCodes::AmountTooHigh)}
   
        execute_token_transfer(amount_won,
            ctx.accounts.reward_vault.to_account_info(), 
            ctx.accounts.player_reward_token_account.to_account_info(),
            ctx.accounts.campaign_auth.to_account_info(),
            ctx.accounts.token_program.to_account_info(), 
            Some(&[&[ctx.accounts.campaign.key().as_ref(),&[ctx.accounts.campaign.auth_bump]]]))?;
            
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
    campaign_player.rewards_claimed += amount_won;

    ctx.accounts.campaign.active_games = ctx.accounts.campaign.active_games.saturating_sub(1);
    ctx.accounts.campaign.total_games +=1;
    ctx.accounts.campaign.rewards_available = ctx.accounts.campaign.rewards_available - amount_won;

    //TODO: fix 
    ctx.accounts.campaign.reserved_rewards = ctx.accounts.campaign.reserved_rewards.saturating_sub(ctx.accounts.campaign.max_rewards_per_game);
    Ok(())
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub house: Box<Account<'info, House>>,

    #[account(mut, has_one=house, has_one=reward_mint)]
    pub campaign: Box<Account<'info, Campaign>>,
    
    /// CHECK: campaign proxy signer
    #[account(seeds=[campaign.key().as_ref()], bump)]
    pub campaign_auth: AccountInfo<'info>,

    #[account(mut, 
        seeds = [
            b"player", 
            campaign.key().as_ref(), 
            &match (&player_nft_metadata, &player_core_nft) {
                (Some(metadata), _) => metadata.mint.to_bytes(),
                (None, Some(core_nft)) => core_nft.key().to_bytes(),
                (None, None) => user.key().to_bytes(),
            }[..]
        ],
        bump)]
    pub campaign_player: Box<Account<'info, CampaignPlayer>>,

    #[account(
        constraint = player_nft_metadata.as_ref().is_some_and(|m| m.mint == player_nft_token_account.mint ), 
        constraint = player_nft_token_account.owner == user.key() @ ErrorCodes::TokenOwnerMismatch, 
        constraint = player_nft_token_account.amount == 1 @ ErrorCodes::OwnerBalanceMismatch,
    )]
    pub player_nft_token_account: Option<Box<Account<'info, TokenAccount>>>,


    #[account(constraint = metadata_contains(&player_nft_metadata,&campaign.nft_config.unwrap().collection).is_ok())]
    pub player_nft_metadata: Option<Box<Account<'info, MetadataAccount>>>,

    /// CHECK: Custom validation for mpl-core asset
    #[account()]
    pub player_core_nft: Option<AccountInfo<'info>>,

    #[account()]
    pub reward_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump, 
        token::mint = reward_mint, 
        token::authority = campaign_auth
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

impl EndGame<'_> {
    pub fn validate_core_nft(&self) -> Result<()> {
        if let Some(core_nft_info) = &self.player_core_nft {
            let nft = SimplifiedAssetV1::from_account_info(core_nft_info)?;
            
            if nft.owner != self.user.key() {
                return err!(ErrorCodes::TokenOwnerMismatch);
            }
            
            if let Some(nft_config) = self.campaign.nft_config {
                if nft.update_authority != UpdateAuthority::Collection(nft_config.collection) {
                    return err!(ErrorCodes::OwnerBalanceMismatch);
                }
            } else {
                return err!(ErrorCodes::InvalidInput);
            }
        }
        
        Ok(())
    }
    pub fn get_player_identity(&self) -> Result<PlayerIdentity> {
        match (self.campaign.nft_config, &self.player_nft_metadata, &self.player_core_nft) {
            (None, None, None) => Ok(PlayerIdentity{
                    identity_type: IdentityType::User,
                    pubkey: self.user.key(),
                }),
            (None, Some(_), None) => err!(ErrorCodes::UnexpectedMetadata),
            (Some(_), None, None) => err!(ErrorCodes::MissingMetadata),
            (Some(_), Some(metadata), None) => Ok(PlayerIdentity{
             identity_type: IdentityType::Nft,
             pubkey: metadata.mint
            }),
            (None, None, Some(nft)) => Ok(PlayerIdentity{
                identity_type: IdentityType::MplCore,
                pubkey: nft.key()
            }),
            (_, _, _) => err!(ErrorCodes::InvalidInput),
        }
    }
}