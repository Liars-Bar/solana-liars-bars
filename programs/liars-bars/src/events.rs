use anchor_lang::prelude::*;

#[event]
pub struct LiarsTableCreated {
    pub table_id: u128,
}

#[event]
pub struct PlayerJoined {
    pub table_id: u128,
    pub player: Pubkey,
}

#[event]
pub struct SuffleCardsForPlayer {
    pub table_id: u128,
    pub player: Pubkey,
    pub next: Pubkey,
}

#[event]
pub struct RoundStarted {
    pub table_id: u128,
}

#[event]
pub struct TableTrun {
    pub table_id: u128,
    pub player: Pubkey,
}

#[event]
pub struct CardPlaced {
    pub table_id: u128,
    pub player: Pubkey,
}

#[event]
pub struct LiarCalled {
    pub table_id: u128,
    pub caller: Pubkey,
}

#[event]
pub struct EmptyBulletFired {
    pub table_id: u128,
    pub player: Pubkey,
}

#[event]
pub struct PlayerEleminated {
    pub table_id: u128,
    pub player: Pubkey,
}
