use bevy::prelude::*;

use crate::grid::{GRID_CELL_SIZE, CELL_BORDER_WIDTH, GridConfig};

#[derive(Component)]
pub struct Tetromino {
    pub letter: TetrominoLetter,
    pub shape: [[bool; 4]; 4], // 4x4 grid for the tetromino shape
    pub position: (usize, usize), // (x, y) position on the grid
    pub rotation: usize, // 0-3 for 0-270 degrees
    pub color: Color,
}

pub enum TetrominoLetter {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}

pub enum TetrominoColor {
    LightBlue,
    DarkBlue,
    Orange,
    Yellow,
    Green,
    Red,
    Magenta,
}
impl TetrominoColor {
    pub fn to_color(&self) -> Color {
        match self {
            TetrominoColor::LightBlue => Color::srgb(0.0, 1.0, 1.0),
            TetrominoColor::DarkBlue => Color::srgb(0.0, 0.0, 1.0),
            TetrominoColor::Orange => Color::srgb(1.0, 0.5, 0.0),
            TetrominoColor::Yellow => Color::srgb(1.0, 1.0, 0.0),
            TetrominoColor::Green => Color::srgb(0.0, 1.0, 0.0),
            TetrominoColor::Red => Color::srgb(1.0, 0.0, 0.0),
            TetrominoColor::Magenta => Color::srgb(1.0, 0.0, 1.0),
        }
    }
}

#[derive(Component)]
pub struct Active {}

#[derive(Component)]
pub struct TetrominoCell {}

pub fn draw_tetromino(
    mut commands: Commands,
    query: Query<&mut Tetromino, With<Active>>,
    grid_config: Res<GridConfig>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>
){
    for tetromino in query.iter() {
        let start_x = tetromino.position.0;
        let start_y = tetromino.position.1;

        for y in 0..4 {
            for x in 0..4 {
                if tetromino.shape[y][x] {
                    let cell_x = grid_config.start_x + (start_x + x) as f32 * GRID_CELL_SIZE;
                    let cell_y = grid_config.start_y + (start_y - y) as f32 * GRID_CELL_SIZE;

                    // Draw the cell
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::default())),
                        MeshMaterial2d(materials.add(tetromino.color)),
                        Transform::from_xyz(cell_x, cell_y, 0.0)
                            .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0)),
                        TetrominoCell {},
                    ));
                }
            }
        }
    }
}
