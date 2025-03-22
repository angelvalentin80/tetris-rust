use bevy::prelude::*;
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH, RedrawGridEvent, redraw_grid};
use crate::tetromino::{Tetromino, TetrominoLetter, draw_tetromino, Active, move_tetromino, detect_lock_position};
use crate::systems::{gravity, lock_in_tetromino};
use crate::resources::{GravityTimer, LockInTimer};

mod grid;
mod tetromino;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<RedrawGridEvent>()
        .add_systems(Startup, (setup, draw_grid).chain())
        .add_systems(Update, (gravity, lock_in_tetromino, move_tetromino, draw_tetromino, detect_lock_position, redraw_grid))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Adding a grid config resource
    commands.insert_resource(GridConfig {
        start_x: -(GRID_WIDTH as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
        start_y: -(GRID_HEIGHT as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
    });
    // Adding our grid resource
    commands.insert_resource(Grid::new());

    // Adding our tetromino resource
    commands.spawn((Tetromino::create_tetromino(TetrominoLetter::I), Active {}));
    // commands.spawn((Tetromino::create_tetromino(TetrominoLetter::O), Active {}));
    // commands.spawn((Tetromino::create_tetromino(TetrominoLetter::J), Active {}));
        
    // Add our gravity resource 
    let gravity_timer = GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating));
    commands.insert_resource(gravity_timer);

    // Add our lock in resource
    let lock_in_timer = LockInTimer(Timer::from_seconds(0.5, TimerMode::Once));
    commands.insert_resource(lock_in_timer);
}