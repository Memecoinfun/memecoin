use anchor_lang::error_code;


#[error_code]
pub enum CurveLaunchpadError {
    #[msg("Global Already Initialized")]
    AlreadyInitialized,
    #[msg("Global Not Initialized")]
    NotInitialized,
    #[msg("Invalid Authority")]
    InvalidAuthority,
    #[msg("Bonding Curve Complete")]
    BondingCurveComplete,
    #[msg("Bonding Curve Not Complete")]
    BondingCurveNotComplete,
    #[msg("Insufficient Tokens")]
    InsufficientTokens,
    #[msg("Insufficient SOL")]
    InsufficientSOL,
    #[msg("Max SOL Cost Exceeded")]
    MaxSOLCostExceeded,
    #[msg("Min SOL Output Exceeded")]
    MinSOLOutputExceeded,
    #[msg("Min buy is 1 Token")]
    MinBuy,
    #[msg("Min sell is 1 Token")]
    MinSell,
    #[msg("Invalid Fee Recipient")]
    InvalidFeeRecipient,
    #[msg("Invalid Withdraw Authority")]
    InvalidWithdrawAuthority,
    #[msg("Wrong wrapped sol mint")]
    WrongWSOLMint, 
    #[msg("Invalid mint address.")]
    InvalidMintAddress,
    #[msg("Already withdraw.")]
    AlreadyWithdraw,
    #[msg("The fee account status is abnormal.")]
    FeeAccountStatusAbnormal,
    #[msg("There is no claim fee.")]
    NotClaimableFee,
    #[msg("Invite account not init.")]
    InviteAccountNotInit,

    #[msg("Invite account error.")]
    InviteAccountError,
    
}