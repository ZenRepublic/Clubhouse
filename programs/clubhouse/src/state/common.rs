use crate::errors::ErrorCodes;
use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;

 pub fn metadata_contains(metadata: &MetadataAccount, pk: &Pubkey) -> bool {
        let collection = &metadata.collection;
        if collection.is_some() && collection.as_ref().unwrap().verified && collection.as_ref().unwrap().key.eq(&pk)  {
            return true;
        }
        let creators = &metadata.creators;
        if creators.is_some() {
            for creator in creators.as_ref().unwrap() {
                if creator.verified && creator.address.eq(&pk) {
                    return true;
                }
            }
        }
        return false;
    }

   pub fn validate_string(s: &str) -> Result<()> {
        if s.is_empty() {
            return Ok(());
        }
        if s.len() > 32 {
            return err!(ErrorCodes::StringTooLong);
        }
        if s.len() < 4 {
            return err!(ErrorCodes::StringTooShort);
        }
        let chars: Vec<char> = s.chars().collect();
        if chars[0].is_ascii_punctuation() {
            return err!(ErrorCodes::StartsWithPunctuation);
        }
        if chars[0] == ' ' {
            return err!(ErrorCodes::StartsWithWhitespace);
        }
        if chars[chars.len() - 1] == ' ' {
            return err!(ErrorCodes::EndsWithWhitespace);
        }
    
        let mut prev_was_space = false;
    
        for &c in &chars {
            if c == ' ' {
                if prev_was_space {
                    return err!(ErrorCodes::ConsecutiveWhitespace);
                }
                prev_was_space = true;
            } else {
                prev_was_space = false;
                if (c as u32) > 0xFFFF || c.is_control() || !is_allowed_char(c) {
                    return err!(ErrorCodes::InvalidCharacter);
                }
            }
        }
    
        Ok(())
    }

    fn is_allowed_char(c: char) -> bool {
        c.is_alphanumeric() || 
        matches!(c, '.' | ',' | '!' | '?' | ':' | ';' | '(' | ')' | '[' | ']' | '{' | '}' | 
                 '\'' | '"' | '-' | '_' | '@' | '#' | '$' | '%' | '&' | '*' | '+' | '=' | 
                 '<' | '>' | '/' | '\\' | '|' | '~' | '^')
    }
    

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrainingConfig {
    pub max_rewards_per_training: u64,
    pub max_club_member_energy: u8,
    pub energy_recharge_minutes: i64,
    pub burn_remaining_tokens: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct HouseConfig {
    pub oracle_key: Pubkey,
    pub campaign_creation_fee: u64,
    pub campaign_manager_discount: u64,
    pub claim_fee: u64,
    pub rewards_tax: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct MatchConfig {
    pub win_tax_basis_points: u64,
    pub match_min_deposit: u64,
}
