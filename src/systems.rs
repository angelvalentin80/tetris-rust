use bevy::prelude::*;
use crate::grid::{Grid, CellState, GRID_WIDTH, RedrawGridEvent};
use crate::tetromino::{Active, SpawnTetrominoEvent, Tetromino, TetrominoCell};
use crate::resources::LockInTimer;

pub fn lock_in_tetromino(
    mut commands: Commands,
    mut grid: ResMut<Grid>, 
    mut redraw_grid_event: EventWriter<RedrawGridEvent>,
    mut spawn_tetromino_event: EventWriter<SpawnTetrominoEvent>,
    mut lock_in_timer: ResMut<LockInTimer>,
    tetromino_query: Query<(Entity, &Tetromino), With<Active>>,
    tetromino_cell_query: Query<(Entity, &TetrominoCell)>,
) {

    if lock_in_timer.0.just_finished() {
        for (entity, tetromino) in tetromino_query.iter() {
            // Lock in the tetromino by updating the grid state
            let start_x = tetromino.position.0;
            let start_y = tetromino.position.1;

            for y in 0..4 {
                for x in 0..4 {
                    if tetromino.shape[y][x] {
                        let index = ((start_y - y as i32) * GRID_WIDTH as i32 + (start_x + x as i32)) as usize;
                        grid.cells[index] = CellState::Filled(tetromino.color);
                    }
                }
            }
            
            commands.entity(entity).remove::<Active>();
            commands.entity(entity).despawn();
        }

        for (entity, _) in tetromino_cell_query.iter() {
            commands.entity(entity).despawn();
        }

        redraw_grid_event.send(RedrawGridEvent);
        spawn_tetromino_event.send(SpawnTetrominoEvent);
        lock_in_timer.0.reset();
    }
}