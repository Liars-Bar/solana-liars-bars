use std::vec;

use anchor_lang::prelude::*;
use inco_lightning::IncoLightning;

use crate::{
    constant::INCO_LIGHTNING_ID,
    error::LiarsBarsError,
    events::RoundStarted,
    helpers::reset_round,
    state::{LiarsTable, Player},
};

#[derive(Accounts)]
#[instruction(table_id:u128)]
pub struct StartRound<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, LiarsTable>,

    #[account(
       mut,
        seeds = [b"player", table_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub players: Account<'info, Player>,

    pub system_program: Program<'info, System>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn handler(ctx: Context<StartRound>, table_id: u128) -> Result<()> {
    let table = &mut ctx.accounts.table;
    // TODO: restore to >= 2 for production
    require!(table.players.len() >= 1, LiarsBarsError::NeedTwoPlayer);

    table.is_open = false;
    table.is_over = false;

    let signer_info = ctx.accounts.signer.to_account_info();
    let inco_info = ctx.accounts.inco_lightning_program.to_account_info();
    table.remaining_bullet = vec![6; table.players.len()];
    table.player_cards_left = vec![5; table.players.len()];
    reset_round(table, &signer_info, &inco_info)?;

    emit!(RoundStarted { table_id });

    Ok(())
}
