use crate::{
    constant::INCO_LIGHTNING_ID,
    error::LiarsBarsError,
    events::{CardPlaced, RoundStarted, TableTrun},
    helpers::reset_round,
    state::{Card, LiarsTable, Player},
};
use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{allow, as_euint128, Allow, Operation},
    program::IncoLightning,
};

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

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, PlaceCards<'info>>, table_id: u128, picked_indexs: Vec<u8>) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player = &mut ctx.accounts.player;
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let signer_info = ctx.accounts.user.to_account_info();
    let signer_key = ctx.accounts.user.key();

    let operation = Operation {
        signer: signer_info.clone(),
    };

    let i = table
        .players
        .iter()
        .position(|&p| p == ctx.accounts.user.key());

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
        let idx = 0;
        let mut values = 0;
        let mut shapes = 0;
        for i in 0..player.cards.len() {
            if (i == sorted_indexs[idx] as usize) {
                table.cards_on_table.push(player.cards.remove(i));
            } else {
                values = values * 100 as u128 + player.cards[i].value.0 as u128;
                shapes = shapes * 100 as u128 + player.cards[i].shape.0 as u128;
            }
        }
        let remaining = &ctx.remaining_accounts;
        let mut remaining_idx = 0;

        let encrypted_shape =
            as_euint128(CpiContext::new(inco.clone(), operation.clone()), shapes)?;

        if remaining_idx < remaining.len() {
            let shape_allowance = remaining[remaining_idx].clone();
            remaining_idx += 1 as usize;

            allow(
                CpiContext::new(
                    inco.clone(),
                    Allow {
                        allowance_account: shape_allowance,
                        signer: signer_info.clone(),
                        allowed_address: signer_info.clone(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                ),
                encrypted_shape.0, // Extract handle from Euint128
                true,
                signer_key,
            )?;
        }

        let encrypted_value =
            as_euint128(CpiContext::new(inco.clone(), operation.clone()), values)?;

        if remaining_idx < remaining.len() {
            let value_allowance = remaining[remaining_idx].clone();
            remaining_idx += 1 as usize;

            allow(
                CpiContext::new(
                    inco.clone(),
                    Allow {
                        allowance_account: value_allowance,
                        signer: signer_info.clone(),
                        allowed_address: signer_info.clone(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                ),
                encrypted_value.0, // Extract handle from Euint128
                true,
                signer_key,
            )?;
        }

        player.card_values = Card {
            shape: encrypted_shape,
            value: encrypted_value,
        };
        // for x in sorted_indexs {
        //     require!(
        //         (x as usize) < player.cards.len(),
        //         LiarsBarsError::NotEligible
        //     );
        // }
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
