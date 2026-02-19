// File: programs/liars_bar_dapp/src/instructions/grant_card_access.rs

use crate::{constant::INCO_LIGHTNING_ID, state::Player};
use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{allow, Allow},
    program::IncoLightning,
};

#[derive(Accounts)]
#[instruction(table_id: u128)]
pub struct GrantCardAccess<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"player", table_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(address = INCO_LIGHTNING_ID)]
    pub inco_lightning_program: Program<'info, IncoLightning>,

    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, GrantCardAccess<'info>>,
    _table_id: u128,
) -> Result<()> {
    let player = &ctx.accounts.player;
    let inco = ctx.accounts.inco_lightning_program.to_account_info();
    let signer_info = ctx.accounts.signer.to_account_info();
    let signer_key = ctx.accounts.signer.key();

    let remaining = &ctx.remaining_accounts;
    let mut remaining_idx = 0;

    for card in &player.cards {
        // Grant access for shape
        if remaining_idx < remaining.len() {
            allow(
                CpiContext::new(
                    inco.clone(),
                    Allow {
                        allowance_account: remaining[remaining_idx].clone(),
                        signer: signer_info.clone(),
                        allowed_address: signer_info.clone(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                ),
                card.shape.0,
                true,
                signer_key,
            )?;
            remaining_idx += 1 as usize;
        }

        // Grant access for value
        if remaining_idx < remaining.len() {
            allow(
                CpiContext::new(
                    inco.clone(),
                    Allow {
                        allowance_account: remaining[remaining_idx].clone(),
                        signer: signer_info.clone(),
                        allowed_address: signer_info.clone(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                ),
                card.value.0,
                true,
                signer_key,
            )?;
            remaining_idx += 1 as usize;
        }
    }

    Ok(())
}
