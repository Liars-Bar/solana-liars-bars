use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{e_rand, Operation},
    IncoLightning,
};

use crate::{
    constant::INCO_LIGHTNING_ID,
    events::RoundStarted,
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
    let player = &mut ctx.accounts.players;
    let table = &mut ctx.accounts.table;
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let operation = Operation {
        signer: ctx.accounts.signer.to_account_info(),
    };

    table.is_open = false;
    table.is_over = false;

    table.deck = vec![vec![false; 13]; 4];

    let cpi_ctx = CpiContext::new(inco.clone(), operation.clone());
    let random_number: u128 = e_rand(cpi_ctx, 0)?.0;

    table.table_card = (random_number % 4) as u8;

    table.suffle_trun = 0;

    emit!(RoundStarted { table_id });

    Ok(())
}
