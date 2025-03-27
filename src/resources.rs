use::bevy::prelude::*;

use std::collections::VecDeque;
use crate::tetromino::TetrominoLetter;

#[derive(Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Resource)]
pub struct LockInTimer(pub Timer);

#[derive(Resource, Debug)] //TODO remove debug??
pub struct TetrominoQueue {
    pub queue: VecDeque<TetrominoLetter>,
}

#[derive(Resource)]
pub struct GameState {
    pub started: bool,
}