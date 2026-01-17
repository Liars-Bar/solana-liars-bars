use crate::constant::ANCHOR_DISCRIMINATOR_SIZE;
use crate::state::Player;
use anchor_lang::prelude::*;
use inco_lightning::cpi::{as_ebool, Operation};
use inco_lightning::IncoLightning;

#[derive(Accounts)]
#[instruction(table_id:u64)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer=user,
        space= ANCHOR_DISCRIMINATOR_SIZE + Player::INIT_SPACE,
        seeds = [b"player",table_id.to_le_bytes().as_ref(),user.key().as_ref()],
        bump
    )]
    pub players: Account<'info, Player>,

    pub system_program: Program<'info, System>,

    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn create_player_handler(ctx: Context<InitializePlayer>, table_id: u64) -> Result<()> {
    let player = &mut ctx.accounts.players;

    player.table_id = table_id;

    player.is_eliminated = false;
    let cpi_ctx = CpiContext::new(
        ctx.accounts.inco_lightning_program.to_account_info(),
        Operation {
            signer: ctx.accounts.user.to_account_info(),
        },
    );

    for i in 0..5 {
        player.placed_cards.push(false);
    }

    Ok(())
}
