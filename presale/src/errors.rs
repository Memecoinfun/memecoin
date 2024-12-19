use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Invalid launch success bps.")]
    InvalidLaunchSuccessFeeBps, // 0x1770
    #[msg("The status of this memecoin is not ongoing.")]
    StatusNotOngoing, // 0x1771
    #[msg("Unsold memecoin is insufficient.")]
    UnsoldTokenInsufficient, // 0x1772
    #[msg("This memecoin sale is already closed.")]
    SaleClosed, // 0x1773
    #[msg("CalculationError.")]
    CalculationError, // 0x1774
    #[msg("Cannot claim when memecoin launched successfully.")]
    CannotClaimWhenLaunchSuccess, // 0x1775
    #[msg("Only can create the raydium pool when launched successfully.")]
    OnlyCreatePoolWhenLaunchSuccess, // 0x1776
    #[msg("Wrong wrapped sol mint")]
    WrongWSOLMint, // 0x1777
    #[msg("Invalid funding raise tier")]
    InvalidFundingRaiseTier, // 0x1778
    #[msg("Cannot claim when not end.")]
    CannotClaimWhenNotEnd, // 0x1779
    #[msg("Claim amount is too small.")]
    ClaimAmountTooSmall, // 0x177a
    #[msg("Only can wrap sol when launched successfully.")]
    OnlyWrapSolWhenLaunchSuccess, // 0x177b
    #[msg("Invalid account.")]
    InvalidAccount, // 0x177c
    #[msg("Invalid mint address.")]
    InvalidMintAddress,
}