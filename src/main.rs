use bevy::{ecs::storage::Resources, prelude::*, state::commands};
use crate::grid::{draw_grid, Grid, GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, CELL_BORDER_WIDTH};
use crate::tetromino::{Tetromino, TetrominoLetter, TetrominoColor, draw_tetromino, Active};
use crate::systems::gravity;
use crate::resources::GravityTimer;

mod grid;
mod tetromino;
mod systems;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, draw_grid, draw_tetromino).chain())
        .add_systems(Update, gravity)
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

    // Adding our tetromino resources
    let spawn_position= (3, 20); 
    commands.spawn((Tetromino {
        letter: TetrominoLetter::I,
        shape: [[true, true, true, true], 
                [false, false, false, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::LightBlue.to_color(),
    },
    ));
    commands.spawn((Tetromino {
        letter: TetrominoLetter::J,
        shape: [[true, false, false, false], 
                [true, true, true, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::DarkBlue.to_color(),
    }, Active {},
    ));
    commands.spawn(Tetromino {
        letter: TetrominoLetter::L,
        shape: [[false, false, true, false], 
                [true, true, true, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::Orange.to_color(),
    });
    commands.spawn(Tetromino {
        letter: TetrominoLetter::O,
        shape: [[true, true, false, false], 
                [true, true, false, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::Yellow.to_color(),
    });
    commands.spawn(Tetromino {
        letter: TetrominoLetter::S,
        shape: [[false, true, true, false], 
                [true, true, false, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::Green.to_color(),
    });
    commands.spawn(Tetromino {
        letter: TetrominoLetter::Z,
        shape: [[true, true, false, false], 
                [false, true, true, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::Red.to_color(),
    });
    commands.spawn(Tetromino {
        letter: TetrominoLetter::T,
        shape: [[false, true, false, false], 
                [true, true, true, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::Magenta.to_color(),
    });

    // Add our gravity resource 
    let gravity_timer = GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating));
    commands.insert_resource(gravity_timer)
    
}