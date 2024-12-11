use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("Signer does not match program authority")]
    ProgramAuthorityMismatch,
    #[msg("Collection Proof is invalid")]
    CollectionProofInvalid,
    #[msg("Collection Key Mismatch")]
    CollectionKeyMismatch,
    #[msg("Caller does not own the token account")]
    TokenOwnerMismatch,
    #[msg("Unexpected metadata account")]
    UnexpectedMetadata,
    #[msg("Expected metadata account, none provided")]
    MissingMetadata,
    #[msg("Caller does not own the NFT")]
    OwnerBalanceMismatch,
    #[msg("Mint is not an NFT")]
    MintNotNft,
    #[msg("Energy depleted")]
    OutOfEnergy,
    #[msg("Energy calculation failed")]
    EnergyCalculationFailed,
    #[msg("Club inactive")]
    ClubInactive,
    #[msg("Invalid input")]
    InvalidInput,
    #[msg("Metadata does not match mint")]
    MetadataMismatch,
    #[msg("Tax above 100%")]
    TaxTooHigh,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Amount too high")]
    AmountTooHigh,
    #[msg("No Authority")]
    NotClubAdmin,
    #[msg("Player already in a match")]
    PlayerAlreadyInMatch,
    #[msg("Player identity mismatch")]
    PlayerIdentityMismatch,
    #[msg("Name starts with punctuation")]
    StartsWithPunctuation,
    #[msg("Name starts with whitespace")]
    StartsWithWhitespace,
    #[msg("Name ends with whitespace")]
    EndsWithWhitespace,
    #[msg("Invalid character in name")]
    InvalidCharacter,
    #[msg("Consecutive whitespace in name")]
    ConsecutiveWhitespace,
    #[msg("The string cannot be longer than 32 bytes")]
    StringTooLong,
    #[msg("The string cannot be shorter than 4 bytes")]
    StringTooShort,
    #[msg("The vault must be empty")]
    NonZeroVault,
    #[msg("Active campaigns exist")]
    ActiveCampaigns,
    #[msg("Campaign is active")]
    ActiveCampaign,
    #[msg("Campaign is expired")]
    CampaignExpired,
    #[msg("Invalid time span")]
    InvalidTimeSpan,
    #[msg("Player is already in a game")]
    PlayerInGame,
    #[msg("Oracle required")]
    OracleRequired,
    #[msg("Oracle mismatch")]
    OracleMismatch,
    #[msg("Game cannot be started due to insufficient rewards")]
    RewardsUnavailable,
    #[msg("No stake on player")]
    NoStake,
}