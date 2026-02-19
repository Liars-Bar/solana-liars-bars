use crate::error::LiarsBarsError;
use crate::{
    constant::INCO_LIGHTNING_ID,
    events::SuffleCardsForPlayer,
    state::{Card, LiarsTable, Player},
};
use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{allow, as_euint128, e_rand, Allow, Operation},
    program::IncoLightning,
};

#[derive(Accounts)]
#[instruction(table_id: u128)]
pub struct SuffleCards<'info> {
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

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,

    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, SuffleCards<'info>>,
    table_id: u128,
) -> Result<()> {
    let player = &mut ctx.accounts.players;
    let table = &mut ctx.accounts.table;
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let signer_info = ctx.accounts.signer.to_account_info();
    let signer_key = ctx.accounts.signer.key();

    let operation = Operation {
        signer: signer_info.clone(),
    };

    let mut idx: u8 = 0;
    for address in table.players.clone() {
        if address == signer_key {
            break;
        }
        idx = idx + 1 as u8;
    }

    require!(idx == table.suffle_trun, LiarsBarsError::NotYourTrunSuffle);

    let cpi_ctx = CpiContext::new(inco.clone(), operation.clone());
    let mut random_number: u128 = e_rand(cpi_ctx, 0)?.0;

    let mut available: Vec<(u128, u128)> = Vec::with_capacity(52);
    for shape in 0u128..4 {
        for value in 0u128..13 {
            if !table.deck[shape as usize][value as usize] {
                available.push((shape, value));
            }
        }
    }

    // remaining_accounts layout:
    // For each card (up to 5): [shape_allowance, value_allowance]
    // Total: 10 accounts max
    let remaining = &ctx.remaining_accounts;
    let mut remaining_idx = 0;

    for _ in 0..5 {
        if available.is_empty() {
            break;
        }

        let card_idx = (random_number % available.len() as u128) as usize;
        random_number = random_number.checked_div(52).unwrap();

        let (shape, value) = available.swap_remove(card_idx);

        // Encrypt shape and value
        let encrypted_shape = as_euint128(CpiContext::new(inco.clone(), operation.clone()), shape)?;
        let encrypted_value = as_euint128(CpiContext::new(inco.clone(), operation.clone()), value)?;

        // Grant decrypt access for shape (using remaining_accounts)
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

        // Grant decrypt access for value (using remaining_accounts)
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

        player.cards.push(Card {
            shape: encrypted_shape,
            value: encrypted_value,
        });
        table.deck[shape as usize][value as usize] = true;
    }

    table.suffle_trun = idx + 1 as u8;

    emit!(SuffleCardsForPlayer {
        table_id,
        player: signer_key,
        next: table.players[((idx as usize + 1) % table.players.len())]
    });

    Ok(())
}
