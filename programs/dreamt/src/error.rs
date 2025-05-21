use anchor_lang::prelude::*;

#[error_code]
pub enum DiamondTokenError {
    #[msg("Invalid multisig threshold")]
    InvalidMultisigThreshold,

    #[msg("Invalid token state")]
    InvalidTokenState,

    #[msg("Invalid blacklist PDA")]
    InvalidBlacklist,

    #[msg("Address is blacklisted")]
    AddressBlacklisted,

    #[msg("Invalid payment token - only USDC accepted")]
    InvalidPaymentToken,

    #[msg("Invalid token owner")]
    InvalidOwner,

    #[msg("Address is already blacklisted")]
    AddressAlreadyBlacklisted,

    #[msg("Address is not blacklisted")]
    AddressNotBlacklisted,

    #[msg("Blacklist is full")]
    BlacklistFull,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Insufficient reserve")]
    InsufficientReserve,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Purchase amount is too small")]
    PurchaseAmountTooSmall,

    #[msg("Purchase amount is too large")]
    PurchaseAmountTooLarge,

    #[msg("Max supply would be exceeded")]
    MaxSupplyExceeded,

    #[msg("Cannot increase max supply")]
    CannotIncreaseMaxSupply,

    #[msg("Invalid max supply")]
    InvalidMaxSupply,

    #[msg("Max supply reduction too large")]
    MaxSupplyReductionTooLarge,

    #[msg("Token operations are paused")]
    ProgramPaused,

    #[msg("Invalid token decimals")]
    InvalidDecimals,

    #[msg("Math operation overflow")]
    MathOverflow,

    #[msg("Token is already paused")]
    AlreadyPaused,

    #[msg("Token is not paused")]
    NotPaused,

    #[msg("Not authorized")]
    NotAuthorized,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid vault owner")]
    InvalidVaultOwner,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Invalid multisig")]
    InvalidMultisig,

    #[msg("Multisig verification failed")]
    MultisigVerificationFailed,

    #[msg("Source address is blacklisted")]
    SourceAddressBlacklisted,

    #[msg("Destination address is blacklisted")]
    DestinationAddressBlacklisted,

    #[msg("Unpause cooldown not elapsed")]
    UnpauseCooldownNotElapsed,

    #[msg("Invalid token version - SPL Token 2022 required")]
    InvalidTokenVersion,

    #[msg("Operation in progress - reentrancy not allowed")]
    ReentrancyNotAllowed,

    #[msg("Operation cooldown not elapsed")]
    OperationCooldownNotElapsed,

    #[msg("Invalid multisig transaction")]
    InvalidMultisigTransaction,

    #[msg("Invalid timestamp")]
    InvalidTimestamp,

    #[msg("Missing multisig signer")]
    MissingMultisigSigner,

    #[msg("Multisig already initialized")]
    MultisigAlreadyInitialized,

    #[msg("Transfer hook error")]
    TransferHookError,

    #[msg("Invalid token program provided")]
    InvalidTokenProgram,
}
