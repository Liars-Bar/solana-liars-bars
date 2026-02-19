use crate::{
    constant::INCO_LIGHTNING_ID,
    error::LiarsBarsError,
    events::{CardPlaced, TableTrun},
    state::{LiarsTable, Player},
};
use anchor_lang::prelude::*;
use inco_lightning::IncoLightning;
use std::{ops::Index, vec};

#[derive(Accounts)]
#[instruction(table_id:u128)]
pub struct PlaceCards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, LiarsTable>,

    #[account(
        mut,
        seeds = [b"player", table_id.to_le_bytes().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn handler(ctx: Context<PlaceCards>, table_id: u128, picked_indexs: Vec<u8>) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player = &mut ctx.accounts.player;

    let mut idx = 0;
    let mut is_exist = false;
    for i in 0..table.players.len() {
        if table.players[i] == ctx.accounts.user.key() {
            idx = i;
            is_exist = true;
            break;
        }
    }
    require!(is_exist, LiarsBarsError::NotEligible);
    require!(
        idx == table.trun_to_play as usize,
        LiarsBarsError::NotYourTrun
    );

    for i in table.cards_on_table.clone() {
        table.cards_on_table.pop();
    }

    for x in picked_indexs {
        table
            .cards_on_table
            .push(player.cards.swap_remove(x as usize));
    }

    table.trun_to_play = (idx as u8 + 1) % table.players.len() as u8;

    emit!(CardPlaced {
        player: ctx.accounts.user.key(),
        table_id
    });

    emit!(TableTrun {
        table_id,
        player: table.players[((idx + 1) % table.players.len()) as usize]
    });

    Ok(())
}
