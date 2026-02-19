use anchor_lang::prelude::*;

#[error_code]
pub enum LiarsBarsError {
    #[msg("Table Already Initialized")]
    TableAlreadyInitialized,
    #[msg("Table is full please join other table")]
    TableIsFull,
    #[msg("Not your trun to call suffle you scripter")]
    NotYourTrunSuffle,
    #[msg("Not Your Trun to play you scripter")]
    NotYourTrun,
    #[msg("You are Not Eligible for this call")]
    NotEligible,
}
