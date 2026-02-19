use crate::constant::{ANCHOR_DISCRIMINATOR_SIZE, INCO_LIGHTNING_ID};
use crate::events::LiarsTableCreated;
use crate::state::LiarsTable;
use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{as_ebool, as_euint128, e_eq, e_rand, e_rem, e_shr, Operation},
    Euint128, IncoLightning,
};

#[derive(Accounts)]
#[instruction(table_id: u128)]
pub struct InitializeTable<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + LiarsTable::INIT_SPACE,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, LiarsTable>,

    pub system_program: Program<'info, System>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn handler(ctx: Context<InitializeTable>, table_id: u128) -> Result<()> {
    let table = &mut ctx.accounts.table;

    let inco = ctx.accounts.inco_lightning_program.to_account_info();

    let sign = ctx.accounts.signer.to_account_info();
    let player = &mut ctx.accounts.signer;
    // Initialize table state
    table.table_id = table_id;
    table.is_open = true;
    table.trun_to_play = 0;
    table.suffle_trun = 0;

    table.deck = vec![vec![false; 13]; 4];

    emit!(LiarsTableCreated { table_id: table_id });

    Ok(())
}
