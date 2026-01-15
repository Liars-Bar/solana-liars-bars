use crate::constant::ANCHOR_DISCRIMINATOR_SIZE;
use crate::state::GameRoom;
use anchor_lang::prelude::*;
// use inco_lightning::types::{Ebool, Euint128};

#[derive(Accounts)]
#[instruction(room_id:u64)]
pub struct InitializeRoom<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + GameRoom::INIT_SPACE,
        seeds = [b"room", room_id.to_le_bytes().as_ref()],
        bump
    )]
    pub room: Account<'info, GameRoom>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeRoom>, room_id: u64) -> Result<()> {
    let room = &mut ctx.accounts.room;
    room.room_id = room_id;
    room.is_open = true;
    room.startgame = false;
    room.players.push(ctx.accounts.signer.key());
    Ok(())
}
