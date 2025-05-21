use anchor_lang::prelude::*;

#[event]
pub struct TokenStateInitialized {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub initial_supply: u64,
    pub max_supply: u64,
    pub multisig: Pubkey,
}

#[event]
pub struct TokensMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub payment_amount: u64,
    pub payment_token: Option<Pubkey>,
}

#[event]
pub struct TokensBurned {
    pub admin: Pubkey,
    pub amount: u64,
    pub refund_amount: u64,
    pub refund_token: Pubkey,
}

#[event]
pub struct ProgramPaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProgramUnpaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MaxSupplyUpdated {
    pub authority: Pubkey,
    pub old_max_supply: u64,
    pub new_max_supply: u64,
}

#[event]
pub struct BlacklistUpdated {
    pub authority: Pubkey,
    pub address: Pubkey,
    pub action: BlacklistAction,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BlacklistAction {
    Added,
    Removed,
}

#[event]
pub struct ItemPurchased {
    pub user: Pubkey,
    pub amount: u64,
    pub item_id: String,
}

#[event]
pub struct ReserveVerified {
    pub total_supply: u64,
    pub reserve_amount: u64,
    pub reserve_token: Pubkey,
}

#[event]
pub struct TransferHookExecuted {
    pub source: Pubkey,
    pub destination: Pubkey,
    pub amount: u64,
}
