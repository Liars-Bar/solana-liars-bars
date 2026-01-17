#![allow(unexpected_cfgs)]
#![allow(unused)]

use anchor_lang::prelude::*;
pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("Cgehp7M8KnHzXC7NNa3C43ECq9DL6GkpERoAFPACSwyF");

#[program]
pub mod liars_bar_dapp {
    use super::*;

    pub fn create_table(ctx: Context<InitializeTable>, table_id: u64) -> Result<()> {
        instructions::CreateTable::create_table_handler(ctx, table_id)
    }

    pub fn create_player(ctx: Context<InitializePlayer>, table_id: u64) -> Result<()> {
        instructions::CreatePlayer::create_player_handler(ctx, table_id)
    }
}
