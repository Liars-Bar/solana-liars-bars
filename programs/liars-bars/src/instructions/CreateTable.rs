use crate::constant::ANCHOR_DISCRIMINATOR_SIZE;
use crate::state::GameTable;
use anchor_lang::prelude::*;
use inco_lightning::{
    cpi::{as_ebool, as_euint128, e_eq, e_rand, e_rem, e_shr, Operation},
    Euint128, IncoLightning,
};

#[derive(Accounts)]
#[instruction(table_id: u64)]
pub struct InitializeTable<'info> {
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

    pub inco_lightning_program: Program<'info, IncoLightning>,
}

pub fn create_table_handler(ctx: Context<InitializeTable>, table_id: u64) -> Result<()> {
    let table = &mut ctx.accounts.table;

    let inco = ctx.accounts.inco_lightning_program.to_account_info();

    let sign = ctx.accounts.signer.to_account_info();

    // Initialize table state
    table.table_id = table_id;
    table.bullet_index = 0;

    // Generate random value for bullet position (or other game logic)
    let mut cpi_ctx = CpiContext::new(
        inco.clone(),
        Operation {
            signer: sign.clone(),
        },
    );
    let mut random = e_rand(cpi_ctx, 0)?.0;

    let empty_bullet1 = random % 6;
    random = random.checked_div(10).unwrap();

    let mut empty_bullet2 = random % 6;

    if empty_bullet1 == empty_bullet2 {
        empty_bullet2 = empty_bullet1.checked_add(1).unwrap();
    }

    for i in 0..6 {
        if i == empty_bullet1 {
            cpi_ctx = CpiContext::new(
                inco.clone(),
                Operation {
                    signer: sign.clone(),
                },
            );
            table.bullet.push(as_ebool(cpi_ctx, false)?.0);
        } else if i == empty_bullet2 {
            cpi_ctx = CpiContext::new(
                inco.clone(),
                Operation {
                    signer: sign.clone(),
                },
            );
            table.bullet.push(as_ebool(cpi_ctx, false)?.0);
        } else {
            cpi_ctx = CpiContext::new(
                inco.clone(),
                Operation {
                    signer: sign.clone(),
                },
            );
            table.bullet.push(as_ebool(cpi_ctx, true)?.0);
        }
    }

    Ok(())
}
