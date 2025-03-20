use bevy::prelude::*;
use crate::tetromino::{TetrominoCell};
use crate::grid::{GRID_CELL_SIZE};
use crate::resources::GravityTimer;

pub fn gravity(
    time: Res<Time>,
    mut tetromino_cell_query: Query<(&mut TetrominoCell, &mut Transform)>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (_, mut transform) in tetromino_cell_query.iter_mut() {
            // Move the tetromino down by one cell each second
            transform.translation.y -= GRID_CELL_SIZE;
        }
    }
}