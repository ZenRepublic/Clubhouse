use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::MetadataAccount, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{errors::{self, ErrorCodes}, execute_token_close, execute_token_transfer, state::{metadata_is_collection, ManagerSlot}, Campaign, House, TokenUse};

pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
    if false &&ctx.accounts.campaign.time_span.is_active(Clock::get()?.unix_timestamp) {
        return err!(errors::ErrorCodes::ActiveCampaign);
    }
    ctx.accounts.house.remove_campaign();

    let campaign_key_bytes = ctx.accounts.campaign.key().to_bytes();
    let bump = [ctx.accounts.campaign.auth_bump];

    let binding = [&[campaign_key_bytes.as_ref(), &bump][..]];
    let seeds = Some(&binding[..]);
    execute_token_transfer(
        ctx.accounts.reward_vault.amount,
        ctx.accounts.reward_vault.to_account_info(),
        ctx.accounts.reward_withdrawal_account.to_account_info(),
        ctx.accounts.campaign_auth.to_account_info(),
        ctx.accounts.reward_token_program.to_account_info(),
        seeds)?;
    execute_token_close(
        ctx.accounts.reward_vault.to_account_info(),
        ctx.accounts.creator.to_account_info(),
        ctx.accounts.campaign_auth.to_account_info(),
        ctx.accounts.reward_token_program.to_account_info(),
        seeds)?;

    let is_paid = ctx.accounts.campaign.token_config.is_some_and(|c| c.token_use == TokenUse::Pay);

    match (&ctx.accounts.game_deposit_vault, &ctx.accounts.game_mint, &ctx.accounts.deposit_withdrawal_account, &ctx.accounts.deposit_token_program, is_paid) {
        (Some(game_deposit_vault), Some(_), Some(withdrawal_account), Some(token_program), true) => {
            execute_token_transfer(
                game_deposit_vault.amount,
                game_deposit_vault.to_account_info(),
                withdrawal_account.to_account_info(),
                ctx.accounts.campaign_auth.to_account_info(),
            token_program.to_account_info(),
            seeds)?;
            execute_token_close(
                game_deposit_vault.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.campaign_auth.to_account_info(),
                token_program.to_account_info(),
                seeds)?;
        },
        (None, None, None, None, false) => {

        },
        _ => return err!(errors::ErrorCodes::InvalidInput),
        
    }

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
        associated_token::token_program = reward_token_program)]
    pub reward_withdrawal_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// the vault where the rewards are held to be claimed

    pub reward_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds=[b"rewards", campaign.key().as_ref()], 
        bump
    )]
    pub reward_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub house: Box<Account<'info, House>>,
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut, token::mint=game_mint, seeds=[b"player_deposit", campaign.key().as_ref()], bump)]
    pub game_deposit_vault: Option<Box<InterfaceAccount<'info, TokenAccount>>>,
    
    #[account(init_if_needed, 
        payer=creator,
        associated_token::authority = creator,
        associated_token::mint = game_mint,
        associated_token::token_program = deposit_token_program
    )]
    pub deposit_withdrawal_account : Option<Box<InterfaceAccount<'info, TokenAccount>>>,

    
    pub game_mint: Option<Box<InterfaceAccount<'info, Mint>>>,

    pub deposit_token_program: Option<Interface<'info, TokenInterface>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub reward_token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,


    /// Optional manager NFT token account
    pub manager_nft_token_account: Option<Box<InterfaceAccount<'info, TokenAccount>>>,

    /// Optional manager NFT metadata
    pub manager_nft_metadata: Option<Box<Account<'info, MetadataAccount>>>,

    #[account(
        mut,
        close=creator,
        seeds=[b"manager_slot", manager_nft_metadata.as_ref().unwrap().mint.key().as_ref()], bump,
        constraint = house.manager_collection.is_some() @ ErrorCodes::InvalidInput,
        constraint = metadata_is_collection(&manager_nft_metadata.as_ref().unwrap(),&house.manager_collection.unwrap()).is_ok() @ ErrorCodes::CollectionProofInvalid

    )]
    pub manager_proof: Option<Account<'info,ManagerSlot>>,
}