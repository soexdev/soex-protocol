use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Admin access required")]
    AdminAccessRequired,

    #[msg("Signer not the sender")]
    InvalidSigner,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Invalid cvt sol account")]
    InvalidCvtSOLAccount,

    #[msg("Cvt sol account not verify")]
    CvtSOLAccountNotVerify,

    #[msg("Already transfer")]
    AlreadyTransfer,

    #[msg("The state is already set")]
    AlreadySet,

    #[msg("Invalid launchpad program")]
    InvalidLaunchpadProgram,

    #[msg("Invalid NFT ID")]
    InvalidNftId,

    #[msg("Maximum supply reached")]
    MaxSupplyReached,

    #[msg("Invalid admin")]
    InvalidAdmin,

    #[msg("Invalid owner")]
    InvalidOwner,

    #[msg("Signature verification failed")]
    SigVerificationFailed,

    #[msg("Invalid copies")]
    InvalidCopies,

    #[msg("Invalid phase")]
    InvalidPhase,

    #[msg("Phase is over")]
    PhaseIsOver,

    #[msg("Phase has not yet started")]
    PhaseNotStarted,

    #[msg("Invalid action")]
    InvalidAction,

    #[msg("Closed value is already set to the provided value")]
    ClosedAlreadySet,

    #[msg("Array index out of bounds")]
    ArrayIndexOutOfBounds,

    #[msg("Invalid max supply")]
    InvalidMaxSupply,

    #[msg("Authority value is already set to the provided value")]
    AuthorityAlreadySet,

    #[msg("ZeroAddressError")]
    ZeroAddressError
}
