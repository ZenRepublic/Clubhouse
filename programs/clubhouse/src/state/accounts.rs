use anchor_lang::prelude::*;

use super::{
    common::{HouseConfig, TrainingConfig},
    MatchConfig,
};

#[account]
/// If the admin is set, the program will check if the caller is the admin, otherwise it should check if the caller is the program authority
pub struct ProgramAdminProof {
    pub program_admin: Pubkey,
}

#[account]
pub struct Club {
    pub house: Pubkey,
    pub manager_mint: Pubkey,
    pub metadata_mint: Option<Pubkey>,
    pub reward_mint: Pubkey,
    pub reward_mint_decimals: u8,
    pub is_active: bool,
    pub member_count: u32,
    pub active_trainings: u32,
    pub active_matches: u32,
    pub reserved: [u8; 128],
    pub training_config: TrainingConfig,
    pub _reserved_training: [u8; 64],
    pub match_config: MatchConfig,
    pub _reserved_match: [u8; 64],
}

#[account]
pub struct Campaign {
    pub bump: u8,
    pub house: Pubkey,
    pub creator: Pubkey,
    pub manager_mint: Option<Pubkey>,
    pub reward_mint: Pubkey,
    pub reward_mint_decimals: u8,
    pub max_rewards_per_game: u64,
    pub rewards_claim_fee: u64,
    pub player_count: u32,
    pub active_games: u32,
    pub total_games: u32,
    pub time_span: TimeSpan,
    pub house_config_snapshot: HouseConfig,
    pub nft_config: Option<NftCampaignConfig>,
    pub unclaimed_sol_fees: u64,
    pub _reserved_config: [u64; 7],
    pub token_config: Option<TokenCampaignConfig>,
    pub _reserved_for_token: [u64; 4],
    pub init_funding: u64,
    pub rewards_available: u64,
    pub reserved_rewards: u64,
    pub slot_created: u64,
    pub campaign_name: String,
}

#[account]
pub struct Match {
    pub club: Pubkey,
    pub members_joined: u8,
    pub wager: u64,
    pub start_time: i64,
    pub reserved: [u8; 64],
}

#[account]
pub struct Duel {
    pub bump: u8,
    pub club_member_a: Pubkey,
    pub a_is_ready: bool,
    pub club_member_b: Pubkey,
    pub b_is_ready: bool,
    pub wager: u64,
    pub start_time: i64,
}

#[account]
pub struct MatchConnection {
    pub game_match: Pubkey,
    pub club_member: Pubkey,
}

#[account]
pub struct House {
    pub bump: u8,
    pub house_admin: Pubkey,
    /// to create CAMPAIGNS
    pub manager_collection: Option<Pubkey>,
    /// paid to create CAMPAIGNS
    pub house_currency: Pubkey,
    /// currency decimals
    pub house_currency_decimals: u8,
    /// total campaigns created
    pub total_campaigns: u32,
    /// open campaigns
    pub open_campaigns: u16,
    /// unique players in house
    pub unique_players: u32,
    /// total games played in house
    pub games_played: u32,
    /// creation fees collected total
    pub unclaimed_sol_fees: u64,
    /// house fees pending withdrawal
    pub unclaimed_house_fees: u64,
    /// house fees pending withdrawal
    pub is_active: bool,

    _reserved1: [u64; 16],

    pub config: HouseConfig,

    _reserved2: [u64; 16],

    pub house_name: String,
}

impl House {
    pub fn initialize(
        &mut self,
        house_admin: Pubkey,
        manager_collection: Option<Pubkey>,
        house_currency: Pubkey,
        house_currency_decimals: u8,
        config: HouseConfig,
        house_name: String,
        bump: u8,
    ) {
        self.house_admin = house_admin;
        self.manager_collection = manager_collection;
        self.house_currency = house_currency;
        self.house_currency_decimals = house_currency_decimals;
        self.config = config;
        self.house_name = house_name;
        self.bump = bump;
        self.is_active = true;
    }

    pub fn teardown(&mut self) {
        self.is_active = false;
        self.open_campaigns = 0;
    }

    pub fn update(&mut self, new_config: HouseConfig) {
        self.config = new_config;
    }
}

impl Campaign {
    pub fn initialize(
        &mut self,
        bump: u8,
        house: Pubkey,
        creator: Pubkey,
        manager_mint: Option<Pubkey>,
        reward_mint: Pubkey,
        reward_mint_decimals: u8,
        max_rewards_per_game: u64,
        rewards_claim_fee: u64,
        time_span: TimeSpan,
        house_config_snapshot: HouseConfig,
        nft_config: Option<NftCampaignConfig>,
        token_config: Option<TokenCampaignConfig>,
        campaign_name: String,
    ) {
        self.bump = bump;
        self.house = house;
        self.creator = creator;
        self.manager_mint = manager_mint;
        self.reward_mint = reward_mint;
        self.reward_mint_decimals = reward_mint_decimals;
        self.max_rewards_per_game = max_rewards_per_game;
        self.rewards_claim_fee = rewards_claim_fee;
        self.time_span = time_span;
        self.house_config_snapshot = house_config_snapshot;
        self.nft_config = nft_config;
        self.token_config = token_config;
        self.campaign_name = campaign_name;
    }

    pub fn teardown(&mut self) {
        // Add any necessary teardown logic here
    }
}

impl Club {
    pub fn initialize(
        &mut self,
        house: Pubkey,
        manager_mint: Pubkey,
        metadata_mint: Option<Pubkey>,
        reward_mint: Pubkey,
        reward_mint_decimals: u8,
        training_config: TrainingConfig,
        match_config: MatchConfig,
    ) {
        self.house = house;
        self.manager_mint = manager_mint;
        self.metadata_mint = metadata_mint;
        self.reward_mint = reward_mint;
        self.reward_mint_decimals = reward_mint_decimals;
        self.is_active = true;
        self.training_config = training_config;
        self.match_config = match_config;
    }

    pub fn teardown(&mut self) {
        self.is_active = false;
    }
}

#[account]
pub struct Activity {
    pub house: Pubkey,
    pub native_mint: Option<Pubkey>,
    // participant counts
    pub minimum_participants: u8,
    pub maximum_participants: u8,

    pub energy_recharge_minutes: i64,
    pub burn_remaining_tokens: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct NftCampaignConfig {
    pub collection: Pubkey,
    pub max_player_energy: u8,
    pub energy_recharge_minutes: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TokenCampaignConfig {
    pub token_address: Pubkey,
    pub energy_price: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct CampaignConstants {
    pub min_game_duration: i64,
    pub max_game_duration: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Duration {
    pub min_duration: i64,
    pub max_duration: i64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TimeSpan {
    pub start_time: i64,
    pub end_time: i64,
}

impl TimeSpan {
    pub fn is_active(&self, now: i64) -> bool {
        self.start_time <= now && self.end_time >= now
    }

    pub fn is_expired(&self, now: i64) -> bool {
        self.end_time < now
    }

    pub fn is_pending(&self, now: i64) -> bool {
        self.start_time > now
    }

    pub fn is_valid(&self) -> bool {
        self.start_time < self.end_time
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct WagerConfig {
    // wager from players
    pub minimum_wager: u64,
    pub maximum_wager: u64,

    /// payout percent - percentage of the wager that is paid out to the winner
    pub wager_payout_percent: u8,
    /// burn percent - percentage of the wager that is burned
    pub wager_burn_percent: u8,
    /// tax percent - percentage of the wager that is paid to the house
    pub wager_tax_percent: u8,
}

#[account]
pub struct CampaignPlayer {
    pub mint: Pubkey,
    pub campaign: Pubkey,
    pub energy: u8,
    pub recharge_start_time: i64,
    pub game_start_time: i64,
    pub games_played: u32,
    pub in_game: bool,
    pub game: Option<Pubkey>,
    pub campaign_slot: u64,
    _reserved: [u8; 24],
}

impl CampaignPlayer {
    pub const SEC_PER_MINUTE: i64 = 60;

    pub fn new<'info>(
        mint: &Pubkey,
        campaign: &Account<'info, Campaign>,
    ) -> Result<CampaignPlayer> {
        let clock = Clock::get()?;
        Ok(CampaignPlayer {
            mint: mint.key(),
            campaign: campaign.key(),
            energy: campaign.nft_config.unwrap().max_player_energy,
            recharge_start_time: clock.unix_timestamp,
            games_played: 0,
            in_game: false,
            game: None,
            game_start_time: 0,
            campaign_slot: campaign.slot_created,
            _reserved: [0; 24],
        })
    }
    
    pub fn recharge_energy(
        &mut self,
        energy_config: &Option<NftCampaignConfig>,
        now_ts: i64,
    ) -> Result<u8> {
        if energy_config.is_none() {
            return Ok(self.energy);
        }
        let config = energy_config.as_ref().unwrap();
        match config.energy_recharge_minutes {
            Some(recharge_minutes) => {
                let recharge_seconds = (recharge_minutes as i64)
                    .checked_mul(CampaignPlayer::SEC_PER_MINUTE)
                    .unwrap();
                let time_passed_since_update =
                    now_ts.checked_sub(self.recharge_start_time).unwrap();
                // whole energy points replenished since last update
                let energy_restored_since_update = time_passed_since_update
                    .checked_div(recharge_seconds)
                    .unwrap();
                // if energy is not full, the last update happened at the last recharge, otherwise it happened now
                let last_recharge_tick_if_not_full = self
                    .recharge_start_time
                    .checked_add(
                        energy_restored_since_update
                            .checked_mul(recharge_seconds)
                            .unwrap(),
                    )
                    .unwrap();
                let estimated_energy = i64::from(self.energy)
                    .checked_add(energy_restored_since_update)
                    .unwrap();
                
                let energy_is_maxed = estimated_energy >= config.max_player_energy.into();
                (self.energy, self.recharge_start_time) = match energy_is_maxed {
                    true => (config.max_player_energy, now_ts),
                    false => (
                        estimated_energy.try_into().unwrap(),
                        last_recharge_tick_if_not_full,
                    ),
                };
                Ok(self.energy)
            }
            None => return Ok(self.energy),
        }
    }

    pub fn spend_energy(&mut self, energy_to_spend: u8) {
        self.energy = self.energy.checked_sub(energy_to_spend).unwrap();
    }
}
