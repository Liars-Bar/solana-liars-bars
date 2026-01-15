use anchor_lang::prelude::*;
// use inco_lightning::types::{Ebool, Euint128};

#[account]
#[derive(InitSpace)]
pub struct GameRoom {
    pub room_id: u64,
    pub is_open: bool,
    // pub player_count: u64,
    pub startgame: bool,
    #[max_len(5)]
    pub players: Vec<Pubkey>,
}

// #[account]
// #[derive(InitSpace)]
// pub struct GameTable {
//     pub room_id: u64,
//     #[max_len(5)]
//     pub latest: Vec<(Euint128, Euint128)>,
//     #[max_len(6)]
//     pub bullet: Vec<Ebool>,
//     // pub bullet_index: u64,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct Player {
//     pub room_id: u64,
//     pub is_eliminated: Ebool,
//     #[max_len(5)]
//     pub cards: Vec<(Euint128, Euint128)>,
//     #[max_len(5)]
//     pub placed_cards: Vec<Ebool>,
// }

// #[account]
// #[drive(InitSpace)]
// pub struct LobbyRoom {
//     #[max_len(50)]
//     pub room_id: Vec<u64>,
// }
