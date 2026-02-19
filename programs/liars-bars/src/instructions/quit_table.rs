use anchor_lang::prelude::*;
use inco_lightning::IncoLightning;

use crate::{
    constant::{ANCHOR_DISCRIMINATOR_SIZE, INCO_LIGHTNING_ID},
    state::{LiarsTable, Player},
};

#[derive(Accounts)]
#[instruction(table_id:u128)]
pub struct QuitTable<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, LiarsTable>,
    // #[account(mut, close = signer, )]
    #[account(
       mut,
        seeds = [b"player", table_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump,
        close = signer
    )]
    pub players: Account<'info, Player>,

    pub system_program: Program<'info, System>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn handler(ctx: Context<QuitTable>, table_id: u128) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player = &mut ctx.accounts.players;
    let playerkey = ctx.accounts.signer.key();

    for card in player.cards.clone() {
        table.deck[card.shape.0 as usize][card.value.0 as usize] = false;
    }

    let mut idx = 0;

    for x in table.players.clone() {
        if x == playerkey {
            break;
        }
        idx = idx + 1 as usize;
    }

    table.players.swap_remove(idx);

    Ok(())
}
