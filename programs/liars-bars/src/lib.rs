#![allow(unexpected_cfgs)]
#![allow(unused)]

use anchor_lang::prelude::*;
pub mod constant;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8SEnxoxd14bJyAb9HQ2mUR3hi4gk8oqty43VVFZCVjTx");

#[program]
pub mod liars_bar_dapp {
    use super::*;

    pub fn create_room(ctx: Context<InitializeRoom>, room_id: u64) -> Result<()> {
        instructions::CreateRoom::handler(ctx, room_id)
    }
}
