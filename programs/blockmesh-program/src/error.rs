use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Numerical overflow")]
    NumericalOverflow,
    #[msg("Latest Provider Node Report Cannot Be Lower Than Previous Report")]
    LatestProviderNodeReportCannotBeLowerThanPreviousReport,
    #[msg("Latest Client Report Cannot Be Lower Than Previous Report")]
    LatestClientReportCannotBeLowerThanPreviousReport,
    #[msg("Signer Is Not A Valid Provider Node")]
    SignerNotProviderNode,
    #[msg("Signer Is Not A Valid Client")]
    ClientNotProviderNode,
    #[msg("Mismatch On Reported Usage")]
    MismatchOnReportedUsage,
    #[msg("Signer mismatch")]
    SignerMismatch,
    #[msg("Invalid data")]
    InvalidData,
    #[msg("Address mismatch")]
    AddressMismatch,
}
