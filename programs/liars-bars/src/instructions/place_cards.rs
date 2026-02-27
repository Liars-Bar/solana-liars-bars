use crate::{
    constant::INCO_LIGHTNING_ID,
    error::LiarsBarsError,
    events::{CardPlaced, RoundStarted, TableTrun},
    helpers::reset_round,
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
    realloc = 8 + LiarsTable::INIT_SPACE,  // define this const on your struct
    realloc::payer = user,
    realloc::zero = false,
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

    // let mut idx = 0;
    // let mut is_exist = false;

    let i = table
        .players
        .iter()
        .position(|&p| p == ctx.accounts.user.key());
    // for i in 0..table.players.len() {
    //     if table.players[i] == ctx.accounts.user.key() {
    //         idx = i;
    //         is_exist = true;
    //         break;
    //     }
    // }
    // require!(i.is_none(), LiarsBarsError::NotEligible);
    let mut idx = i.ok_or(LiarsBarsError::NotEligible)?;

    require!(
        idx == table.trun_to_play as usize,
        LiarsBarsError::NotYourTrun
    );

    require!(
        table.suffle_trun as usize >= table.players.len(),
        LiarsBarsError::ShuffleNotComplete
    );

    if player.cards.len() != 0 {
        if table.cards_on_table.len() > 0 {
            table.cards_on_table.clear();
        }
        let picked_count = picked_indexs.len();
        // Sort descending so that removing by index doesn't invalidate earlier indices,
        // and use `remove` (not `swap_remove`) to keep the same order as the client.
        let mut sorted_indexs = picked_indexs;
        sorted_indexs.sort_by(|a, b| b.cmp(a));
        for x in sorted_indexs {
            require!(
                (x as usize) < player.cards.len(),
                LiarsBarsError::NotEligible
            );
            table.cards_on_table.push(player.cards.remove(x as usize));
        }
        table.player_cards_left[idx] = table.player_cards_left[idx]
            .checked_sub(picked_count as u8)
            .ok_or(LiarsBarsError::NotEligible)?;
        emit!(CardPlaced {
            player: ctx.accounts.user.key(),
            table_id
        });
    }

    let st = idx;
    idx = idx.checked_add(1).unwrap() % table.players.len();
    while idx != st {
        if table.player_cards_left[idx] != 0 {
            break;
        }
        idx = idx.checked_add(1).unwrap() % table.players.len();
    }

    if (st == idx) {
        let signer_info = ctx.accounts.user.to_account_info();
        let inco_info = ctx.accounts.inco_lightning_program.to_account_info();
        reset_round(table, &signer_info, &inco_info)?;
        emit!(RoundStarted { table_id });
    }

    table.trun_to_play = idx as u8;

    emit!(TableTrun {
        table_id,
        player: table.players[idx.checked_add(1).unwrap() % table.players.len()]
    });

    Ok(())
}
