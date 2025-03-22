use bevy::prelude::*;
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH, RedrawGridEvent, redraw_grid};
use crate::tetromino::{draw_tetromino, move_tetromino, detect_lock_position, spawn_tetromino, gravity, SpawnTetrominoEvent};
use crate::systems::{lock_in_tetromino};
use crate::resources::{GravityTimer, LockInTimer, TetrominoQueue, GameState};
use crate::queue::{shuffle_tetrominoes_into_queue, detect_bag_low, BagLowEvent};
use crate::game_manager::{GameStartEvent, detect_start_game};

mod grid;
mod tetromino;
mod systems;
mod resources;
mod queue;
mod game_manager;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Events
        .add_event::<RedrawGridEvent>()
        .add_event::<BagLowEvent>()
        .add_event::<GameStartEvent>()
        .add_event::<SpawnTetrominoEvent>()
        // Systems
        .add_systems(Startup, 
            (
                setup, 
                draw_grid,
            ).chain())
        .add_systems(Update, 
            (
                detect_start_game,
                gravity, 
                lock_in_tetromino, 
                move_tetromino, 
                draw_tetromino, 
                detect_lock_position, 
                redraw_grid, 
                spawn_tetromino,
                detect_bag_low,
                shuffle_tetrominoes_into_queue,
            ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Add game state resource
    commands.insert_resource(GameState { started: false });

    // Adding a grid config resource
    commands.insert_resource(GridConfig {
        start_x: -(GRID_WIDTH as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
        start_y: -(GRID_HEIGHT as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
    });
    // Adding our grid resource
    commands.insert_resource(Grid::new());

    // Add our gravity resource 
    let gravity_timer = GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating));
    commands.insert_resource(gravity_timer);

    // Add our lock in resource
    let lock_in_timer = LockInTimer(Timer::from_seconds(0.5, TimerMode::Once));
    commands.insert_resource(lock_in_timer);

    // Add our tetromino queue resource
    commands.insert_resource(TetrominoQueue{queue: vec![]});
}