use bevy::prelude::*;
use std::collections::VecDeque;
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH, RedrawGridEvent, redraw_grid, CheckForLinesEvent, check_for_lines, reset_grid};
use crate::tetromino::{draw_tetromino, move_tetromino, detect_lock_position, spawn_tetromino, gravity, SpawnTetrominoEvent, draw_ghost_piece, RedrawGhostCellsEvent, LockInTetrominoEvent, draw_next_piece_text, draw_next_piece, spawn_next_piece, SpawnNextPieceEvent, despawn_active_tetromino, despawn_next_piece, gravity_seconds_for_level, update_gravity_timer};
use crate::systems::lock_in_tetromino;
use crate::resources::{GravityTimer, LockInTimer, TetrominoQueue, GameState};
use crate::queue::{shuffle_tetrominoes_into_queue, detect_bag_low, BagLowEvent, restart_queue};
use crate::game_manager::{GameStartEvent, detect_start_game, detect_restart_game, GameRestartEvent};
use crate::scoring::{Scoring, draw_level_and_score, RedrawLevelAndScoreEvent, reset_level_and_score, LevelUpEvent};

mod grid;
mod tetromino;
mod systems;
mod resources;
mod queue;
mod game_manager;
mod scoring;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Events
        .add_event::<RedrawGridEvent>()
        .add_event::<BagLowEvent>()
        .add_event::<GameStartEvent>()
        .add_event::<SpawnTetrominoEvent>()
        .add_event::<RedrawGhostCellsEvent>()
        .add_event::<CheckForLinesEvent>()
        .add_event::<LockInTetrominoEvent>()
        .add_event::<SpawnNextPieceEvent>()
        .add_event::<GameRestartEvent>()
        .add_event::<RedrawLevelAndScoreEvent>()
        .add_event::<LevelUpEvent>()
        // Systems
        .add_systems(Startup, 
            (
                setup, 
                draw_grid,
            ).chain())
        .add_systems(Update, 
            (
                (detect_start_game, redraw_grid, shuffle_tetrominoes_into_queue, spawn_tetromino, draw_tetromino, draw_ghost_piece, draw_next_piece_text, draw_level_and_score, spawn_next_piece, draw_next_piece).chain(),
                (gravity, lock_in_tetromino, move_tetromino, detect_lock_position, detect_bag_low, check_for_lines, update_gravity_timer),
                (restart_queue, detect_restart_game, despawn_active_tetromino, reset_grid, despawn_next_piece, reset_level_and_score)
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
    let initial_time = gravity_seconds_for_level(1);
    let gravity_timer = GravityTimer(Timer::from_seconds(initial_time, TimerMode::Repeating));
    commands.insert_resource(gravity_timer);

    // Add our lock in resource
    let lock_in_timer = LockInTimer(Timer::from_seconds(0.5, TimerMode::Once));
    commands.insert_resource(lock_in_timer);

    // Add our tetromino queue resource
    commands.insert_resource(TetrominoQueue{queue: VecDeque::new()});

    // Add our scoring resource
    commands.insert_resource(Scoring{level: 1, score: 0, lines_cleared: 0});
}