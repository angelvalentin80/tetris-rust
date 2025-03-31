use bevy::prelude::*;
use crate::grid::GridPlugin;
use crate::tetromino::TetrominoPlugin;
use crate::game_manager::GameManagerPlugin;
use crate::queue::QueuePlugin;
use crate::scoring::ScoringPlugin;
use crate::tips::TipsPlugin;

mod grid;
mod tetromino;
mod queue;
mod game_manager;
mod scoring;
mod tips;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Tetris".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1920., 1080.).into(),
                    ..default()
                }),
                ..default()}),
                GridPlugin,
                TetrominoPlugin,
                GameManagerPlugin,
                QueuePlugin,
                ScoringPlugin,
                TipsPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}