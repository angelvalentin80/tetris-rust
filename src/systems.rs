use bevy::prelude::*;
use crate::game_manager::GameRestartEvent;
use crate::grid::{Grid, CellState, RedrawGridEvent, get_vec_index_from_grid_coordinates, CheckForLinesEvent};
use crate::tetromino::{Active, GhostCell, LockInTetrominoEvent, RedrawGhostCellsEvent, SpawnTetrominoEvent, Tetromino, TetrominoCell};
use crate::resources::LockInTimer;

pub fn lock_in_tetromino(
    mut commands: Commands,
    mut grid: ResMut<Grid>, 
    mut redraw_grid_event: EventWriter<RedrawGridEvent>,
    mut spawn_tetromino_event: EventWriter<SpawnTetrominoEvent>,
    mut lock_in_timer: ResMut<LockInTimer>,
    tetromino_query: Query<(Entity, &Tetromino), With<Active>>,
    tetromino_cell_query: Query<(Entity, &TetrominoCell)>,
    ghost_cell_query: Query<(Entity, &GhostCell)>,
    mut redraw_ghost_cells_event: EventReader<RedrawGhostCellsEvent>, 
    mut check_for_lines_event: EventWriter<CheckForLinesEvent>,
    mut lock_in_tetromino_event: EventReader<LockInTetrominoEvent>
) {
    if !lock_in_tetromino_event.is_empty(){
        lock_in_tetromino_event.clear();
        if lock_in_timer.0.finished() {
            for (entity, tetromino) in tetromino_query.iter() {
                // Lock in the tetromino by updating the grid state
                let start_x = tetromino.position.0;
                let start_y = tetromino.position.1;

                for y in 0..4 {
                    for x in 0..4 {
                        if tetromino.shape[y][x] {
                            let index = get_vec_index_from_grid_coordinates(start_x + x as i32, start_y - y as i32);
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

            if !redraw_ghost_cells_event.is_empty() {
                redraw_ghost_cells_event.clear();
                for (entity, _) in ghost_cell_query.iter() {
                    commands.entity(entity).despawn();
                }
            }

            check_for_lines_event.send(CheckForLinesEvent);
            redraw_grid_event.send(RedrawGridEvent);
            spawn_tetromino_event.send(SpawnTetrominoEvent);
            lock_in_timer.0.reset();
        }
    } 
}

pub fn reset_lock_in_timer(
    mut game_restart_event: EventReader<GameRestartEvent>,
    mut lock_in_timer: ResMut<LockInTimer>

){
    if !game_restart_event.is_empty(){
        game_restart_event.clear();
        lock_in_timer.0.reset();
    }
}