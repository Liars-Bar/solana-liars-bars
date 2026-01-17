use anchor_lang::prelude::*;

#[error_code]
pub enum LiarsBarsError {
    #[msg("Table Already Initialized")]
    TableAlreadyInitialized,
    #[msg("Table is full please join other table")]
    TableIsFull,
}
