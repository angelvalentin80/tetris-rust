use bevy::prelude::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::tetromino::{TetrominoLetter, SpawnTetrominoEvent};
use crate::resources::TetrominoQueue;
use crate::game_manager::GameStartEvent;

#[derive(Event)]
pub struct BagLowEvent;

pub fn shuffle_tetrominoes_into_queue(
    mut tetromino_queue: ResMut<TetrominoQueue>,
    mut bag_low_event: EventReader<BagLowEvent>,
    mut game_start_event: EventReader<GameStartEvent>,
    mut spawn_tetromino_event: EventWriter<SpawnTetrominoEvent>,
) {

    if !bag_low_event.is_empty() || !game_start_event.is_empty() {

        bag_low_event.clear();
        game_start_event.clear();

        let mut tetrominoes = vec![
            TetrominoLetter::I,
            TetrominoLetter::O,
            TetrominoLetter::T,
            TetrominoLetter::S,
            TetrominoLetter::Z,
            TetrominoLetter::J,
            TetrominoLetter::L,
        ];

        tetrominoes.shuffle(&mut thread_rng());
        tetromino_queue.queue.extend(tetrominoes);
        spawn_tetromino_event.send(SpawnTetrominoEvent);
    }
}

pub fn detect_bag_low(
    tetromino_queue: Res<TetrominoQueue>,
    mut bag_low_event: EventWriter<BagLowEvent>,
) {
    if tetromino_queue.queue.len() == 1 {
        bag_low_event.send(BagLowEvent);
    }
}