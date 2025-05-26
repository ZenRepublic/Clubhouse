use crate::errors::ErrorCodes;
use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;

pub fn metadata_is_collection(metadata: &MetadataAccount, pk: &Pubkey) -> Result<()> {
    let collection = &metadata.collection;
    if collection.is_some() && collection.as_ref().unwrap().verified && collection.as_ref().unwrap().key.eq(&pk)  {
        return Ok(());
    }
    let creators = &metadata.creators;
    if creators.is_some() {
        for creator in creators.as_ref().unwrap() {
            if creator.verified && creator.address.eq(&pk) {
                return Ok(());
            }
        }
    }
    return err!(ErrorCodes::MetadataMismatch);
}
pub fn string_len_borsh(text: &String) -> usize {
    4 + text.len()
}

pub fn string_option_len(text: &Option<String>) -> usize {
    1 + 
    text.as_ref().map_or(0, |s| string_len_borsh(s))
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    Uninitialized,
    AssetV1,
    HashedAssetV1,
    PluginHeaderV1,
    PluginRegistryV1,
    CollectionV1,
}

impl Key {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Key::Uninitialized),
            1 => Some(Key::AssetV1),
            2 => Some(Key::HashedAssetV1),
            3 => Some(Key::PluginHeaderV1),
            4 => Some(Key::PluginRegistryV1),
            5 => Some(Key::CollectionV1),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpdateAuthority {
    None,
    Address(Pubkey),
    Collection(Pubkey),
}

#[derive(Clone, Debug)]
pub struct SimplifiedAssetV1 {
    pub key: Key,
    pub owner: Pubkey,
    pub update_authority: UpdateAuthority,
}

impl SimplifiedAssetV1 {
    pub fn from_account_info(account_info: &AccountInfo) -> Result<Self> {
        let data = account_info.data.borrow();
        
        if data.len() < 34 {
            return Err(error!(ErrorCode::AccountDidNotDeserialize));
        }
        
        let key = Key::from_u8(data[0])
            .ok_or(error!(ErrorCode::AccountDidNotDeserialize))?;
        
        if key != Key::AssetV1 {
            return Err(error!(ErrorCode::AccountDidNotDeserialize));
        }
        
        let owner = Pubkey::new_from_array(data[1..33].try_into().unwrap());
        
        let update_authority = match data[33] {
            0 => UpdateAuthority::None,
            1 => {
                if data.len() < 66 {
                    return Err(error!(ErrorCode::AccountDidNotDeserialize));
                }
                let pubkey = Pubkey::new_from_array(data[34..66].try_into().unwrap());
                UpdateAuthority::Address(pubkey)
            },
            2 => {
                if data.len() < 66 {
                    return Err(error!(ErrorCode::AccountDidNotDeserialize));
                }
                let pubkey = Pubkey::new_from_array(data[34..66].try_into().unwrap());
                UpdateAuthority::Collection(pubkey)
            },
            _ => return Err(error!(ErrorCode::AccountDidNotDeserialize)),
        };
        
        Ok(SimplifiedAssetV1 {
            key,
            owner,
            update_authority,
        })
    }
}