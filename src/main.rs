use bevy::prelude::*;
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH};
use crate::tetromino::{Tetromino, TetrominoLetter, draw_tetromino, Active, move_tetromino};
use crate::systems::gravity;
use crate::resources::GravityTimer;

mod grid;
mod tetromino;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_grid).chain())
        .add_systems(Update, (gravity, move_tetromino, draw_tetromino))
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
        
    // Add our gravity resource 
    let gravity_timer = GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating));
    commands.insert_resource(gravity_timer)
}