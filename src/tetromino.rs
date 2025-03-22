use bevy::prelude::*;

use crate::grid::{GridConfig, CELL_BORDER_WIDTH, GRID_CELL_SIZE, GRID_WIDTH};

#[derive(Component, Clone)]
pub struct Tetromino {
    pub shape: [[bool; 4]; 4], // 4x4 grid for the tetromino shape
    pub position: (usize, usize), // (x, y) position on the grid
    pub rotation: usize, // 0-3 for 0-270 degrees
    pub color: Color,
}

impl Tetromino {
    pub fn create_tetromino(letter: TetrominoLetter) -> Self {
        let shape = match letter {
            TetrominoLetter::I => [[true, true, true, true], 
                                   [false, false, false, false], 
                                   [false, false, false, false], 
                                   [false, false, false, false]],
            TetrominoLetter::J => [[true, false, false, false], 
                                   [true, true, true, false], 
                                   [false, false, false, false], 
                                   [false, false, false, false]],
            TetrominoLetter::L => [[false, false, true, false], 
                                   [true, true, true, false], 
                                   [false, false, false, false], 
                                   [false, false, false, false]],
            TetrominoLetter::O => [[true, true, false, false], 
                                   [true, true, false, false], 
                                   [false, false, false, false], 
                                   [false, false, false, false]],
            TetrominoLetter::S => [[false,true,true,false], 
                                   [true,true,false,false], 
                                   [false,false,false,false], 
                                   [false,false,false,false]],
            TetrominoLetter::Z => [[true,true,false,false], 
                                   [false,true,true,false], 
                                   [false,false,false,false], 
                                   [false,false,false,false]],
            TetrominoLetter::T => [[false,true,false,false],
                                   [true,true,true,false],
                                   [false,false,false,false],
                                   [false,false,false,false]],
        };
        let color = match letter {
            TetrominoLetter::I => TetrominoColor::LightBlue.to_color(),
            TetrominoLetter::J => TetrominoColor::DarkBlue.to_color(),
            TetrominoLetter::L => TetrominoColor::Orange.to_color(),
            TetrominoLetter::O => TetrominoColor::Yellow.to_color(),
            TetrominoLetter::S => TetrominoColor::Green.to_color(),
            TetrominoLetter::Z => TetrominoColor::Red.to_color(),
            TetrominoLetter::T => TetrominoColor::Magenta.to_color(),
        };
        Self {
            shape,
            position: (3 , 20), // Spawn position
            rotation: 0,
            color,
        }
    }

    pub fn rotate_tetromino_shape_clockwise(&self) -> [[bool; 4]; 4] {
        let mut new_shape = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_shape[x][3 - y] = self.shape[y][x];
            }
        }
        new_shape
    }

    pub fn rotate_tetromino_shape_counter_clockwise(&self) -> [[bool; 4]; 4] {
        let mut new_shape = [[false; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                new_shape[3 - x][y] = self.shape[y][x];
            }
        }
        new_shape
    }

    pub fn get_shape_width(&self) -> usize {
        let mut width = 0;
        for x in 0..4 {
            for y in 0..4 {
                if self.shape[y][x] {
                    width = x + 1;
                }
            }
        }
        width
    }
    pub fn get_shape_height(&self) -> usize {
        let mut height = 0;
        for y in 0..4 {
            for x in 0..4 {
                if self.shape[y][x] {
                    height = y + 1;
                }
            }
        }
        height
    }
}

#[derive(Clone)]
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
pub struct TetrominoCell {}

#[derive(Component)]
pub struct Active {}

#[derive(Component)]
pub struct NeedsRedraw();

pub fn draw_tetromino(
    mut commands: Commands,
    tetromino_query: Query<(Entity, &mut Tetromino), (With<Active>, With<NeedsRedraw>)>,
    tetromino_cell_query: Query<(Entity, &mut TetrominoCell)>, 
    grid_config: Res<GridConfig>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>
){
    // Clear the previous tetromino cells
    if !tetromino_query.is_empty() {
        for (entity, _) in tetromino_cell_query.iter() {
            commands.entity(entity).despawn();
        }
    }
    
    for (entity, tetromino ) in tetromino_query.iter() {
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
        commands.entity(entity).remove::<NeedsRedraw>(); // Remove the NeedsRedraw component after drawing 
    }
}

pub fn move_tetromino(
    mut commands: Commands,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (entity, mut tetromino) in tetromino.iter_mut() {

        if tetromino.position.0 > 0 {
            if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                tetromino.position.0 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
            } 
        }

        if tetromino.position.0 < GRID_WIDTH - tetromino.get_shape_width() {
            if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                tetromino.position.0 += 1;
                commands.entity(entity).insert(NeedsRedraw {});
            } 
        }

        if tetromino.position.1 > tetromino.get_shape_height() - 1 {
            if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                tetromino.position.1 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
            }
        }

        if tetromino.position.1 > tetromino.get_shape_height() - 1 && tetromino.position.0 < GRID_WIDTH - tetromino.get_shape_width() {
            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                tetromino.rotation = (tetromino.rotation + 1) % 4; // Rotate the tetromino
                tetromino.shape = tetromino.rotate_tetromino_shape_clockwise(); // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
            }

            if keyboard_input.just_pressed(KeyCode::ControlLeft) {
                tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                tetromino.shape = tetromino.rotate_tetromino_shape_counter_clockwise(); // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
            }
        }
    }
}
