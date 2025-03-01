use anchor_lang::prelude::*;

use crate::errors::ErrorCodes;

#[account]
#[derive(InitSpace)]
/// If the admin is set, the program will check if the caller is the admin, otherwise it should check if the caller is the program authority
pub struct ProgramAdminProof {
    pub program_admin: Pubkey,
}

#[account]

#[derive(InitSpace)]
pub struct Campaign {
    pub auth_bump: u8, 
    pub house: Pubkey,
    pub creator: Pubkey,
    //unused
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
    pub _reserved_for_token: [u64; 3],
    pub rewards_available: u64,
    pub reserved_rewards: u64,
    #[max_len(32)]
    pub campaign_name: String,
    #[max_len(200)]
    pub uri: Option<String>,
}

#[account]
#[derive(InitSpace)]
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

    #[max_len(32)]
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
    ) -> Result<()>{
        crate::common::validate_string(&house_name)?;
        self.house_admin = house_admin;
        self.manager_collection = manager_collection;
        self.house_currency = house_currency;
        self.house_currency_decimals = house_currency_decimals;
        self.config = config;
        self.house_name = house_name;
        self.bump = bump;
        self.is_active = true;
        Ok(())
    }

    pub fn update(&mut self, new_config: HouseConfig) {
        self.config = new_config;
    }

    pub fn add_campaign(&mut self) {
        self.total_campaigns += 1;
        self.open_campaigns += 1;
    }

    pub fn remove_campaign(&mut self) {
        self.open_campaigns -= 1;
    }
}


#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub struct HouseConfig {
    pub oracle_key: Pubkey,
    pub campaign_creation_fee: u64,
    pub campaign_manager_discount: u64,
    pub claim_fee: u64,
    pub rewards_tax: u64,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub struct NftCampaignConfig {
    pub collection: Pubkey,
    pub max_player_energy: u8,
    pub energy_recharge_minutes: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub struct TokenCampaignConfig {
    pub spending_mint: Pubkey,
    pub energy_price: u64,
    pub spending_mint_decimals: u8,
    pub token_use: TokenUse,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum TokenUse {
    Stake,
    Burn,
    Pay,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct CampaignConstants {
    pub min_game_duration: i64,
    pub max_game_duration: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub struct Duration {
    pub min_duration: i64,
    pub max_duration: i64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
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

#[account]
#[derive(InitSpace)]
pub struct CampaignPlayer {
    pub player_identity: PlayerIdentity,
    pub campaign: Pubkey,
    pub house: Pubkey,
    pub energy: u8,
    pub recharge_start_time: i64,
    pub game_start_time: i64,
    pub games_played: u32,
    pub in_game: bool,
    pub rewards_claimed: u64,
    pub stake_info: Option<StakeInfo>,
}

impl CampaignPlayer{
    pub fn get_identity(&self) -> PlayerIdentity {
        self.player_identity
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, InitSpace)]
pub struct StakeInfo{
    pub amount: u64,
    pub campaign_end_time: i64,
    pub staked_mint: Pubkey,
    pub staked_mint_decimals: u8,
    #[max_len(32)]
    pub campaign_name: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub enum IdentityType{
    None,
    Nft,
    User,
    MplCore,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug, InitSpace)]
pub struct PlayerIdentity{
    pub identity_type: IdentityType,
    pub pubkey: Pubkey
}




impl PlayerIdentity {
    pub fn key(&self) -> Option<Pubkey> {
        match self.identity_type {
            IdentityType::None => None,
            IdentityType::Nft => Some(self.pubkey),
            IdentityType::User => Some(self.pubkey),
            IdentityType::MplCore => Some(self.pubkey),
        }
    }
    
}

impl CampaignPlayer {
    pub const SEC_PER_MINUTE: i64 = 60;

    pub fn new<'info>(
        identity: PlayerIdentity,
        campaign: &Account<'info, Campaign>,
    ) -> Result<CampaignPlayer> {
        let clock = Clock::get()?;
        Ok(CampaignPlayer {
            player_identity: identity,
            campaign: campaign.key(),
            house: campaign.house,
            energy: campaign.nft_config.map_or(0, |c| c.max_player_energy),
            recharge_start_time: clock.unix_timestamp,
            games_played: 0,
            in_game: false,
            game_start_time: 0,
            rewards_claimed: 0,
            stake_info: {
                if campaign.token_config.is_some_and(|c| c.token_use == TokenUse::Stake) {
                    Some(StakeInfo {
                        amount: 0,
                        campaign_end_time: campaign.time_span.end_time,
                        staked_mint: campaign.token_config.unwrap().spending_mint,
                        staked_mint_decimals: campaign.token_config.unwrap().spending_mint_decimals,
                        campaign_name: campaign.campaign_name.clone(),
                    })
                } else {
                    None
                }
            },

        })
    }
    
    pub fn recharge_energy(
        &mut self,
        energy_config: &Option<NftCampaignConfig>,
        now_ts: i64,
    ) -> Result<u8> {
        match energy_config {
            None => return Ok(self.energy),
            Some(config) => {
                match config.energy_recharge_minutes {
                    None => return Ok(self.energy),
                    Some(recharge_minutes) => {
                        if self.energy >= config.max_player_energy {
                            return Ok(self.energy);
                        }
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
                        msg!("energy recharged, remaining: {}", self.energy);
                        Ok(self.energy)
                    },
                }
            },
        }
        
        
    }

    pub fn spend_energy(&mut self, energy_to_spend: u8) -> Result<()> {
        match self.player_identity.identity_type {
            IdentityType::None => err!(ErrorCodes::InvalidInput),
            IdentityType::Nft | 
            IdentityType::MplCore => {
                self.energy = self.energy.checked_sub(energy_to_spend).unwrap();
                msg!("energy spent, remaining: {}", self.energy);
                Ok(())
            },
            IdentityType::User => Ok(()),

        }
        
    }
}
