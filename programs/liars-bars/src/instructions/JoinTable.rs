use crate::constant::ANCHOR_DISCRIMINATOR_SIZE;
use crate::error::LiarsBarsError;
use crate::state::GameTable;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct JoinTable<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + GameTable::INIT_SPACE,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, GameTable>,

    pub system_program: Program<'info, System>,
}

pub fn join_table_handler(ctx: Context<JoinTable>, table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;
    let player = &mut ctx.accounts.signer;
    require!(table.players.len() > 5, LiarsBarsError::TableIsFull);
    table.players.push(player.key());
    
    Ok(())
}
