use bevy::prelude::*;
use crate::grid::{Grid, CellState, GRID_WIDTH, RedrawGridEvent};
use crate::tetromino::{Active, NeedsRedraw, Tetromino, TetrominoCell};
use crate::resources::{GravityTimer, LockInTimer};

pub fn gravity(
    mut commands: Commands,
    time: Res<Time>,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (entity, mut tetromino) in tetromino.iter_mut() {

        if tetromino.position.1 > tetromino.get_shape_height() - 1{
                tetromino.position.1 -= 1;
                // Add NeedsRedraw component to tetromino to trigger redraw
                commands.entity(entity).insert(NeedsRedraw {});
            }
        }
    }
}

pub fn lock_in_tetromino(
    mut commands: Commands,
    mut grid: ResMut<Grid>, 
    mut redraw_grid_event_writer: EventWriter<RedrawGridEvent>,
    lock_in_timer: Res<LockInTimer>,
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
                        let index = (start_y - y) * GRID_WIDTH + (start_x + x);
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

        redraw_grid_event_writer.send(RedrawGridEvent);
    }
}