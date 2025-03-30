use bevy::prelude::*;
use std::collections::VecDeque;
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH, RedrawGridEvent, redraw_grid, CheckForLinesEvent, check_for_lines, reset_grid};
use crate::tetromino::{draw_tetromino, move_tetromino, detect_lock_position, spawn_tetromino, gravity, SpawnTetrominoEvent, draw_ghost_piece, RedrawGhostCellsEvent, LockInTetrominoEvent, draw_next_piece_text, draw_next_piece, spawn_next_piece, SpawnNextPieceEvent, despawn_active_tetromino, despawn_next_piece, gravity_seconds_for_level, update_gravity_timer, reset_gravity_timer, lock_in_tetromino, reset_lock_in_timer, maybe_lock_in_tetromino};
use crate::resources::{GravityTimer, LockInTimer, TetrominoQueue, GameState};
use crate::queue::{shuffle_tetrominoes_into_queue, detect_bag_low, BagLowEvent, restart_queue};
use crate::game_manager::{GameStartEvent, detect_start_game, detect_restart_game, GameRestartEvent, GameLoseEvent, spawn_lose_text, animate_lose_text, reset_lose_text};
use crate::scoring::{Scoring, draw_level_and_score, RedrawLevelAndScoreEvent, reset_level_and_score, LevelUpEvent};
use crate::tips::{draw_game_tips, DrawGameTipsEvent, toggle_game_tips};

mod grid;
mod tetromino;
mod resources;
mod queue;
mod game_manager;
mod scoring;
mod tips;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Tetris".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1920., 1080.).into(),
                    ..default()
                }),
                ..default()
        }))
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
        .add_event::<GameLoseEvent>()
        .add_event::<DrawGameTipsEvent>()
        // Systems
        .add_systems(Startup, 
            (
                setup, 
                draw_grid
            ).chain())
        .add_systems(Update, 
            (
                (detect_start_game, redraw_grid, shuffle_tetrominoes_into_queue, spawn_tetromino, draw_tetromino, draw_ghost_piece, draw_next_piece_text, draw_level_and_score, spawn_next_piece, draw_next_piece).chain(),
                (gravity, detect_lock_position, lock_in_tetromino, move_tetromino, detect_bag_low, check_for_lines, update_gravity_timer, maybe_lock_in_tetromino, toggle_game_tips, draw_game_tips),
                (restart_queue, detect_restart_game, despawn_active_tetromino, reset_grid, despawn_next_piece, reset_level_and_score, reset_lock_in_timer, reset_gravity_timer, spawn_lose_text, animate_lose_text, reset_lose_text)
            ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut draw_game_tips_event: EventWriter<DrawGameTipsEvent>
) {
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

    // Adding our help text
    draw_game_tips_event.send(DrawGameTipsEvent);
}