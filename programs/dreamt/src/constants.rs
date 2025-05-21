/// DREAMT Token Constants
/// Configuration for token, pricing, security, and program behavior

/// Token configuration
pub const DECIMALS: u8 = 6;
/// Initial supply in raw token units - 8 million tokens
pub const INITIAL_SUPPLY: u64 = 8_000_000; 
/// Max supply in raw token units - 100 million tokens
pub const MAX_SUPPLY: u64 = 100_000_000;

/// Price configuration
/// 0.8 USDC per token (considering 6 decimals)
pub const TOKEN_PRICE_USDC: u64 = 8 * 10u64.pow(5); 
/// 10 USDC minimum purchase (12.5 tokens minimum)
pub const MIN_PURCHASE_USDC: u64 = 10 * 10u64.pow(6); 
/// 100,000 USDC maximum purchase (as per README)
pub const MAX_PURCHASE_USDC: u64 = 100_000 * 10u64.pow(6);

/// Security configuration
/// Maximum number of addresses in blacklist
pub const MAX_BLACKLIST_SIZE: usize = 100;
/// Cooldown between operations (reentrancy protection)
pub const OPERATION_COOLDOWN: i64 = 1; 

/// Timing constants
/// 15 minutes cooldown period after pause before unpause is allowed
pub const UNPAUSE_COOLDOWN: i64 = 15 * 60; 

/// PDA seeds
pub const TOKEN_STATE_SEED: &[u8] = b"token_state";
pub const BLACKLIST_SEED: &[u8] = b"blacklist";
pub const VAULT_OWNER_SEED: &[u8] = b"vault_owner";
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint_authority";
pub const MULTISIG_SEED: &[u8] = b"multisig";

/// Multisig configuration - 3 of 5 signers required
pub const MULTISIG_THRESHOLD: u64 = 3;
pub const MULTISIG_OWNERS: usize = 5;

/// Additional security constants
/// Number of confirmations required for operations
pub const CONFIRMATION_THRESHOLD: u8 = 1;

/// Anti-whale measures
/// Maximum token transfer amount
pub const MAX_TRANSFER_AMOUNT: u64 = 1_000_000 * 10u64.pow(6); // 1M tokens

/// Emergency cooldown for critical operations (in seconds)
pub const EMERGENCY_COOLDOWN: i64 = 24 * 60 * 60; // 24 hours
