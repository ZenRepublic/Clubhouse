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

declare_id!("C1ubv5AC5w7Eh3iHpEt2BXZ1g3eARQtMRgmE2AXfznSg");

#[program]
pub mod clubhouse {

    use super::*;

    pub fn create_house(ctx: Context<CreateHouse>, manager_collection: Option<Pubkey>, house_config: HouseConfig, house_name: String) -> Result<()> {
        house::create::create_house(ctx, manager_collection, house_config, house_name)
    }

    pub fn update_house(ctx: Context<UpdateHouse>, house_config: HouseConfig) -> Result<()> {
        house::update::update_house(ctx, house_config)
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

    pub fn create_campaign(ctx: Context<CreateCampaign>, campaign_name: String, custom_data: Option<String>, fund_amount: u64, max_rewards_per_game: u64, player_claim_price: u64, time_span: TimeSpan, nft_config: Option<NftCampaignConfig>, token_config: Option<TokenCampaignConfig>) -> Result<()> {
        campaign::create_campaign(ctx, campaign_name, custom_data, fund_amount, max_rewards_per_game, player_claim_price, time_span, nft_config, token_config)
    }

    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        campaign::close_campaign(ctx)
    }

    pub fn start_game_with_nft(ctx: Context<StartGameWithNft>) -> Result<()> {
        game::start_game_with_nft(ctx)
    }

    pub fn end_game_with_nft(ctx: Context<EndGameWithNft>, amount_won: u64) -> Result<()> {
        game::end_game_with_nft(ctx, amount_won)
    }




}