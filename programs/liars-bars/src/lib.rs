#![allow(unexpected_cfgs)]
#![allow(unused)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
pub mod constant;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
use instructions::*;

declare_id!("F618XAoLrCWU7vx5ccd9HB1x85ttjqWwb77FG4TSVWE6");

#[program]
pub mod liars_bar_dapp {
    use super::*;

    pub fn create_table(ctx: Context<InitializeTable>, table_id: u128) -> Result<()> {
        instructions::create_table::handler(ctx, table_id)
    }

    pub fn join_table(ctx: Context<JoinTable>, table_id: u128, character_id: String) -> Result<()> {
        instructions::join_table::handler(ctx, table_id, character_id)
    }

    pub fn start_round(ctx: Context<StartRound>, table_id: u128) -> Result<()> {
        instructions::start_rounds::handler(ctx, table_id)
    }

    pub fn suffle_cards<'info>(
        ctx: Context<'_, '_, '_, 'info, SuffleCards<'info>>,
        table_id: u128,
    ) -> Result<()> {
        instructions::suffle_cards::handler(ctx, table_id)
    }

    pub fn place_cards(
        ctx: Context<PlaceCards>,
        table_id: u128,
        picked_indexs: Vec<u8>,
    ) -> Result<()> {
        instructions::place_cards::handler(ctx, table_id, picked_indexs)
    }

    pub fn quit_table(ctx: Context<QuitTable>, table_id: u128) -> Result<()> {
        instructions::quit_table::handler(ctx, table_id)
    }

    pub fn grant_card_access<'info>(
        ctx: Context<'_, '_, '_, 'info, GrantCardAccess<'info>>,
        table_id: u128,
    ) -> Result<()> {
        grant_card_access::handler(ctx, table_id)
    }
}
