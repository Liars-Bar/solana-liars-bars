use crate::constant::{ANCHOR_DISCRIMINATOR_SIZE, INCO_LIGHTNING_ID};
use crate::error::LiarsBarsError;
use crate::events::PlayerJoined;
use crate::state::{Card, LiarsTable, Player};
use anchor_lang::prelude::*;
use inco_lightning::cpi::{as_euint128, e_rand, Operation};
use inco_lightning::IncoLightning;

#[derive(Accounts)]
#[instruction(table_id: u128)]
pub struct JoinTable<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"table", table_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub table: Account<'info, LiarsTable>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + Player::INIT_SPACE,
        seeds = [b"player", table_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub players: Account<'info, Player>,

    pub system_program: Program<'info, System>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn handler(ctx: Context<JoinTable>, table_id: u128, character_id: String) -> Result<()> {
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let operation = Operation {
        signer: ctx.accounts.signer.to_account_info(),
    };
    let table = &mut ctx.accounts.table;
    let player = &mut ctx.accounts.signer;
    require!(table.players.len() < 5, LiarsBarsError::TableIsFull);
    table.players.push(player.key());

    let player = &mut ctx.accounts.players;
    player.character_id = character_id;
    player.table_id = table_id;

    emit!(PlayerJoined {
        table_id,
        player: ctx.accounts.signer.key()
    });

    Ok(())
}
