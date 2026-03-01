use anchor_lang::prelude::*;
use inco_lightning::types::{Ebool, Euint128};
const EUINT128_SIZE: usize = 32;
const EBOOL_SIZE: usize = 16;
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Card {
    pub shape: Euint128,
    pub value: Euint128,
}

impl anchor_lang::Space for Card {
    const INIT_SPACE: usize = EUINT128_SIZE + EUINT128_SIZE as usize;
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DeckRow {
    pub values: Vec<Ebool>,
}

impl anchor_lang::Space for DeckRow {
    const INIT_SPACE: usize = 4 + (13 * EBOOL_SIZE as usize) as usize;
}
#[account]
#[derive(InitSpace)]
pub struct LiarsTable {
    pub table_id: u128, // uniq number to discribe the account
    pub table_card: u8, // a shape for which players can lia
    #[max_len(5)]
    pub cards_on_table: Vec<Card>, // the cards which player can draw and put on the table which can be maximum 5 cards
    #[max_len(5)]
    pub remaining_bullet: Vec<u8>, // the revolver data of every players
    pub is_open: bool, // this discribe the table is open for join
    pub is_over: bool,
    #[max_len(5)]
    pub players: Vec<Pubkey>, // the pubkey of the players
    #[max_len(4)]
    pub deck: Vec<DeckRow>, // the 52 cards deck out of we can draw the cards
    pub trun_to_play: u8, // this tell us which player is now to play
    pub suffle_trun: u8,  // this tell us which players trun to suffle the cards on table
    #[max_len(5)]
    pub player_cards_left: Vec<u8>,
}

#[account]
#[derive(InitSpace)]
pub struct Player {
    #[max_len(50)]
    pub character_id: String,
    pub table_id: u128, // this player account of which table
    #[max_len(5)]
    pub cards: Vec<Card>, // the player get 5 cards for which he has to lie and not get catch
    pub card_values: Card,
}
