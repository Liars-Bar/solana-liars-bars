use crate::{
    constant::INCO_LIGHTNING_ID,
    state::{LiarsTable, Player},
};
use anchor_lang::prelude::*;
use inco_lightning::IncoLightning;

#[derive(Accounts)]
#[instruction(table_id:u128)]
pub struct ShotFired<'info> {
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

pub fn handler(ctx: Context<ShotFired>, table_id: u128) -> Result<()> {

    
    Ok(())
}
