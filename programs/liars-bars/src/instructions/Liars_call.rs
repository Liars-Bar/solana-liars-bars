use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{e_rand, Operation},
    IncoLightning,
};

use crate::{
    constant::INCO_LIGHTNING_ID,
    error::LiarsBarsError,
    events::{EmptyBulletFired, GameOver, GameWinner, PlayerEleminated, RoundStarted},
    helpers::reset_round,
    state::{LiarsTable, Player},
};

#[derive(Accounts)]
#[instruction(table_id:u128)]
pub struct LiarsCall<'info> {
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

pub fn handler(ctx: Context<LiarsCall>, table_id: u128) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let mut idx = 0;
    let mut is_exist = false;
    for player in table.players.clone() {
        if player == ctx.accounts.signer.key() {
            is_exist = true;
            break;
        }
        idx = idx + 1 as usize;
    }
    require!(is_exist, LiarsBarsError::NotEligible);
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let operation = Operation {
        signer: ctx.accounts.signer.to_account_info(),
    };
    let mut match_count = 0;

    let number_of_cards = table.cards_on_table.len().clone();

    for card in table.cards_on_table.clone() {
        if card.shape.0 == table.table_card as u128 {
            match_count = match_count + 1 as usize;
        }
    }

    let cpi_ctx = CpiContext::new(inco.clone(), operation.clone());

    if number_of_cards != match_count {
        if (idx == 0) {
            idx = table.players.len() - 1 as usize;
        } else {
            idx = idx - 1 as usize;
        }
    }

    let signer_info = ctx.accounts.signer.to_account_info();
    let inco_info = ctx.accounts.inco_lightning_program.to_account_info();

    if table.remaining_bullet[idx] == 1 {
        table.remaining_bullet.swap_remove(idx);
        emit!(PlayerEleminated {
            player: table.players[idx],
            table_id
        });
        table.players.swap_remove(idx);
        table.player_cards_left.swap_remove(idx);
        table.trun_to_play = table.trun_to_play % table.players.len() as u8;
        if table.players.len() == 1 {
            table.is_over = true;
            emit!(GameWinner {
                player: table.players[0],
                table_id
            });
            emit!(GameOver { table_id });
        } else {
            // Still >1 player, start new round
            reset_round(table, &signer_info, &inco)?;
            emit!(RoundStarted { table_id });
        }
        return Ok(());
    }

    let bullet = e_rand(cpi_ctx, 0)?.0 % 2;
    if bullet == 1 {
        emit!(PlayerEleminated {
            player: table.players[idx],
            table_id
        });
        table.players.swap_remove(idx);
        table.remaining_bullet.swap_remove(idx);
        table.player_cards_left.swap_remove(idx);
        table.trun_to_play = table.trun_to_play % table.players.len() as u8;
        if table.players.len() == 1 {
            table.is_over = true;
            emit!(GameWinner {
                player: table.players[0],
                table_id
            });
            emit!(GameOver { table_id });
        } else {
            // Still >1 player, start new round
            reset_round(table, &signer_info, &inco)?;
            emit!(RoundStarted { table_id });
        }
    } else {
        emit!(EmptyBulletFired {
            player: table.players[idx],
            table_id
        });
        table.remaining_bullet[idx] = table.remaining_bullet[idx] - 1 as u8;
        // Player survived, start new round
        reset_round(table, &signer_info, &inco)?;
        emit!(RoundStarted { table_id });
    }

    Ok(())
}
