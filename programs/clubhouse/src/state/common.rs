use crate::errors::ErrorCodes;
use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;

 pub fn metadata_contains(metadata: &MetadataAccount, pk: &Pubkey) -> Result<()> {
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
    

