use anchor_lang::prelude::*;
use inco_lightning::types::{Ebool, Euint128};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Card {
    pub shape: u128,
    pub value: u128,
}

#[account]
#[derive(InitSpace)]
pub struct GameTable {
    pub table_id: u64,
    #[max_len(5)]
    pub latest: Vec<Card>,
    #[max_len(6)]
    pub bullet: Vec<u128>,
    pub bullet_index: u64,
    pub is_open: bool,
    #[max_len(5)]
    pub players: Vec<Pubkey>,
    #[max_len(4, 13)]
    pub deck: Vec<Vec<u128>>,
}

#[account]
#[derive(InitSpace)]
pub struct Player {
    pub table_id: u64,
    pub is_eliminated: bool,
    #[max_len(5,2)]
    pub cards: Vec<Vec<u128>>,
    #[max_len(5)]
    pub placed_cards: Vec<bool>,
}

