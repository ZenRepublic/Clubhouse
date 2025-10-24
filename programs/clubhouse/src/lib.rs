use anchor_lang::prelude::*;
pub mod state;
pub mod errors;
pub mod instructions;
use instructions::*;
use state::*;
use house::*;
use campaign::*;
use game::*;
use program_admin::*;
use player::*;

declare_id!("C1ubv5AC5w7Eh3iHpEt2BXZ1g3eARQtMRgmE2AXfznSg");

#[program]
pub mod clubhouse {

    use super::*;

    pub fn create_house(ctx: Context<CreateHouse>, manager_collection: Option<Pubkey>, house_config: HouseConfig, house_name: String, uri: Option<String>) -> Result<()> {
        house::create::create_house(ctx, manager_collection, house_config, house_name, uri)
    }

    pub fn update_house(ctx: Context<UpdateHouse>, house_config: HouseConfig, uri: Option<String>) -> Result<()> {
        house::update::update_house(ctx, house_config, uri)
    }

    pub fn withdraw_house_fees(ctx: Context<WithdrawHouseFees>) -> Result<()> {
        house::withdraw::withdraw_house_fees(ctx)
    }

    pub fn close_house(ctx: Context<CloseHouse>) -> Result<()> {
        house::close::close_house(ctx)
    }

    pub fn add_program_admin(ctx: Context<AddProgramAdmin>) -> Result<()> {
        program_admin::add::add_program_admin(ctx)
    }

    pub fn create_campaign(ctx: Context<CreateCampaign>, campaign_name: String, custom_data: Option<String>, fund_amount: u64, max_rewards_per_game: u64, player_claim_price: u64, time_span: TimeSpan, nft_config: Option<NftCampaignConfig>, token_config: Option<TokenCampaignConfig>, burn_remainder: bool) -> Result<()> {
        campaign::create_campaign(ctx, campaign_name, custom_data, fund_amount, max_rewards_per_game, player_claim_price, time_span, nft_config, token_config, burn_remainder)
    }

    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        campaign::close_campaign(ctx)
    }

    pub fn start_game(ctx: Context<StartGame>) -> Result<()> {
        game::start_game(ctx)
    }

    pub fn end_game(ctx: Context<EndGame>, amount_won: u64) -> Result<()> {
        game::end_game(ctx, amount_won)
    }

    pub fn claim_stake(ctx: Context<ClaimStake>) -> Result<()> {
        player::claim_stake(ctx)
    }




}