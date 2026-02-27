use anchor_lang::prelude::*;
use inco_lightning::cpi::{e_rand, Operation};
use inco_lightning::types::Ebool;

use crate::state::{DeckRow, LiarsTable};

/// Shared logic to reset the table for a new round.
/// Called from both `start_rounds` and `Liars_call`.
pub fn reset_round<'a>(
    table: &mut LiarsTable,
    signer: &AccountInfo<'a>,
    inco_program: &AccountInfo<'a>,
) -> Result<()> {
    let operation = Operation {
        signer: signer.clone(),
    };
    table.player_cards_left = vec![5; table.players.len()];
    table.deck = vec![DeckRow { values: vec![Ebool::default(); 13] }; 4];
    table.cards_on_table.clear();
    table.suffle_trun = 0;
    table.trun_to_play = 0;
    // table.remaining_bullet = vec![6; table.players.len()];

    let cpi_ctx = CpiContext::new(inco_program.clone(), operation);
    let random_number: u128 = e_rand(cpi_ctx, 0)?.0;
    table.table_card = (random_number % 4) as u8;

    Ok(())
}
