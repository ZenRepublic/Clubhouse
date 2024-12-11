use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors::ErrorCodes, execute_token_transfer, state::CampaignPlayer, StakeInfo};

pub fn claim_stake(ctx: Context<ClaimStake>) -> Result<()> {
    require!(!ctx.accounts.campaign_player.in_game, ErrorCodes::PlayerInGame);
    require!(ctx.accounts.campaign_player.stake_info.is_some(), ErrorCodes::NoStake);
    let campaign = ctx.accounts.campaign_player.campaign;

    match ctx.accounts.campaign_player.stake_info.as_mut() {
        Some(stake_info) => {
            
            execute_token_transfer(
                stake_info.amount,
                ctx.accounts.game_deposit_vault.to_account_info(),
                ctx.accounts.stake_recipient_account.to_account_info(),
                ctx.accounts.campaign_auth.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                Some(&[&[campaign.as_ref(), &[ctx.bumps.campaign_auth]]]))?;

            stake_info.amount = 0;
            stake_info.campaign_end_time = 0;
            stake_info.staked_mint = System::id();
            stake_info.staked_mint_decimals = 0;
            stake_info.campaign_name = "".to_string();
            ctx.accounts.campaign_player.stake_info = None;
            Ok(())
        },
        None => err!(ErrorCodes::NoStake),
    }
}

#[derive(Accounts)]
pub struct ClaimStake<'info>{
    #[account(mut, realloc=8+CampaignPlayer::INIT_SPACE-StakeInfo::INIT_SPACE, realloc::payer = user, realloc::zero=true)]
    pub campaign_player: Box<Account<'info, CampaignPlayer>>,
    #[account(mut)]
    pub user: Signer<'info>,
    ///CHECK: auth
    #[account(seeds=[campaign_player.campaign.as_ref()], bump)]
    pub campaign_auth: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"player_deposit", campaign_player.campaign.as_ref()], 
        bump
    )]
    pub game_deposit_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed, 
        payer=user, 
        associated_token::authority=user, 
        associated_token::mint = game_deposit_mint, 
        associated_token::token_program = token_program)]
    pub stake_recipient_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub game_deposit_mint: Box<InterfaceAccount<'info, Mint>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}