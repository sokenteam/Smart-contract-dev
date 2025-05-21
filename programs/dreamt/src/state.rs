use anchor_lang::prelude::*;
use crate::{error::DiamondTokenError, constants::OPERATION_COOLDOWN};

/// Token state account storing program configuration and state
/// Optimized storage layout with robust security features
#[account]
#[derive(Debug)]
pub struct TokenState {
    pub authority: Pubkey,         // 32 bytes
    pub mint: Pubkey,              // 32 bytes
    pub total_supply: u64,         // 8 bytes
    pub max_supply: u64,           // 8 bytes
    pub is_paused: bool,           // 1 byte
    pub last_pause_timestamp: i64, // 8 bytes
    pub multisig: Pubkey,          // 32 bytes
    pub vault_owner: Pubkey,       // 32 bytes - PDA that owns the vault
    pub bump: u8,                  // 1 byte
    pub in_operation: bool,        // 1 byte - reentrancy guard
    pub last_operation_timestamp: i64, // 8 bytes - operation cooldown
}

impl Default for TokenState {
    fn default() -> Self {
        Self {
            authority: Pubkey::default(),
            mint: Pubkey::default(),
            total_supply: 0,
            max_supply: 0,
            is_paused: false,
            last_pause_timestamp: 0,
            multisig: Pubkey::default(),
            vault_owner: Pubkey::default(),
            bump: 0,
            in_operation: false,
            last_operation_timestamp: 0,
        }
    }
}

impl TokenState {
    pub const LEN: usize = 163; // 8 + 32 + 32 + 8 + 8 + 1 + 8 + 32 + 32 + 1 + 1 + 8

    /// Start an operation with reentrancy protection
    /// Enhanced timing attack protection and secure cooldown checks
    #[inline(always)]
    pub fn start_operation(&mut self) -> Result<()> {
        // Check if already in an operation
        require!(!self.in_operation, DiamondTokenError::ReentrancyNotAllowed);
        
        // Get current timestamp with error handling
        let current_time = Clock::get()?.unix_timestamp;
        
        // Perform checked subtraction for cooldown calculation
        let time_diff = current_time
            .checked_sub(self.last_operation_timestamp)
            .ok_or(DiamondTokenError::MathOverflow)?;
            
        // Verify cooldown has passed
        require!(
            time_diff >= OPERATION_COOLDOWN,
            DiamondTokenError::OperationCooldownNotElapsed
        );

        // Set reentrancy guard and update timestamp
        self.in_operation = true;
        self.last_operation_timestamp = current_time;
        
        Ok(())
    }

    /// End an operation and clear reentrancy guard
    #[inline(always)]
    pub fn end_operation(&mut self) {
        self.in_operation = false;
    }

    /// Check if the provided address is an admin
    #[inline(always)]
    pub fn is_admin(&self, admin: &Pubkey) -> bool {
        self.authority == *admin
    }
    
    /// Update total supply with checked math to prevent overflows
    #[inline(always)]
    pub fn update_total_supply_add(&mut self, amount: u64) -> Result<()> {
        self.total_supply = self.total_supply
            .checked_add(amount)
            .ok_or(DiamondTokenError::MathOverflow)?;
        Ok(())
    }
    
    /// Subtract from total supply with checked math to prevent underflows
    #[inline(always)]
    pub fn update_total_supply_sub(&mut self, amount: u64) -> Result<()> {
        self.total_supply = self.total_supply
            .checked_sub(amount)
            .ok_or(DiamondTokenError::MathOverflow)?;
        Ok(())
    }
}

/// Blacklist account storing addresses that are not allowed to interact with the token
/// Optimized for faster lookup and memory efficiency
#[account]
#[derive(Default, Debug)]
pub struct Blacklist {
    pub addresses: Vec<Pubkey>, // up to MAX_BLACKLIST_SIZE
    pub bump: u8,
}

impl Blacklist {
    pub fn space() -> usize {
        8 + 4 + (32 * crate::constants::MAX_BLACKLIST_SIZE) + 1  // More explicit calculation
    }
    
    /// Optimized contains check for better gas efficiency
    #[inline(always)]
    pub fn contains(&self, address: &Pubkey) -> bool {
        self.addresses.contains(address)
    }
    
    /// Safe add method with blacklist size validation
    #[inline(always)]
    pub fn add(&mut self, address: Pubkey) -> Result<()> {
        // Check if address is already blacklisted
        require!(
            !self.addresses.contains(&address),
            DiamondTokenError::AddressAlreadyBlacklisted
        );
        
        // Check if blacklist is full
        require!(
            self.addresses.len() < crate::constants::MAX_BLACKLIST_SIZE,
            DiamondTokenError::BlacklistFull
        );
        
        // Add address to blacklist
        self.addresses.push(address);
        Ok(())
    }
    
    /// Safe remove method with existence validation
    #[inline(always)]
    pub fn remove(&mut self, address: &Pubkey) -> Result<()> {
        if let Some(index) = self.addresses.iter().position(|x| x == address) {
            // Use swap_remove for gas efficiency (doesn't preserve order but is O(1))
            self.addresses.swap_remove(index);
            Ok(())
        } else {
            err!(DiamondTokenError::AddressNotBlacklisted)
        }
    }
}
