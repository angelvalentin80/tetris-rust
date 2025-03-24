use bevy::prelude::*;

use crate::grid::{GridConfig, CELL_BORDER_WIDTH, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT};
use crate::resources::{TetrominoQueue, LockInTimer, GravityTimer};

#[derive(Component, Clone)]
pub struct Tetromino {
    pub shape: [[bool; 4]; 4], // 4x4 grid for the tetromino shape
    pub position: (i32, i32), // (x, y) position on the grid
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
    // pub fn get_shape_width(&self) -> usize {
    //     let mut width = 0;
    //     for x in 0..4 {
    //         for y in 0..4 {
    //             if self.shape[y][x] {
    //                 width = x + 1;
    //             }
    //         }
    //     }
    //     width
    // }
    // pub fn get_shape_height(&self) -> usize {
    //     let mut height = 0;
    //     for y in 0..4 {
    //         for x in 0..4 {
    //             if self.shape[y][x] {
    //                 height = y + 1;
    //             }
    //         }
    //     }
    //     height
    // }
}

#[derive(Clone, Debug)] // TODO remove debug??
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

#[derive(Event)]
pub struct SpawnTetrominoEvent;

pub fn spawn_tetromino(
    mut commands: Commands,
    mut tetromino_queue: ResMut<TetrominoQueue>,
    mut spawn_tetromino_event: EventReader<SpawnTetrominoEvent>,
) {
    if !spawn_tetromino_event.is_empty() {
        spawn_tetromino_event.clear();

        commands.spawn((
            Tetromino::create_tetromino(tetromino_queue.queue.pop().unwrap()),
            Active {},
            NeedsRedraw {}
        ));
        
    }
}

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
                    let cell_x = grid_config.start_x + (start_x + x as i32) as f32 * GRID_CELL_SIZE;
                    let cell_y = grid_config.start_y + (start_y - y as i32) as f32 * GRID_CELL_SIZE;

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
    mut lock_in_timer: ResMut<LockInTimer>,
    mut gravity_timer: ResMut<GravityTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    for (entity, mut tetromino) in tetromino.iter_mut() {

        // Move Left
        if !is_tetromino_hit_left_wall(&tetromino) {
            if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                tetromino.position.0 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
                lock_in_timer.0.reset(); // Reset the lock-in timer when moving left
            } 
        }

        // Move Right 
        if !is_tetromino_hit_right_wall(&tetromino){
            if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                tetromino.position.0 += 1;
                commands.entity(entity).insert(NeedsRedraw {});
                lock_in_timer.0.reset(); // Reset the lock-in timer when moving right 
            } 
        }

        // Move Down 
        if !is_tetromino_hit_floor(&tetromino) {
            if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                tetromino.position.1 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
                gravity_timer.0.reset();
            }
        }

        // Rotate Clockwise 
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            println!("Attempting to rotate clockwise");
            let new_shape = tetromino.rotate_tetromino_shape_clockwise();
            if !is_collision(&tetromino.position, &new_shape) {
                println!("No collision");
                tetromino.rotation = (tetromino.rotation + 1) % 4; // Rotate the tetromino
                tetromino.shape = new_shape; // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
            } else {
                println!("Collision detected doing some calculations");
                // Adjust position if collision detected
                tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                tetromino.shape = new_shape; // Rotate the shape
                adjust_position(&mut tetromino);
                commands.entity(entity).insert(NeedsRedraw {});
            }
            lock_in_timer.0.reset(); // Reset the lock-in timer when moving right 
        }

        // Rotate Counter Clockwise 
        if keyboard_input.just_pressed(KeyCode::ControlLeft) {
            println!("Attempting to rotate counter clockwise");
            let new_shape = tetromino.rotate_tetromino_shape_counter_clockwise();
            if !is_collision(&tetromino.position, &new_shape) {
                println!("No collision");
                tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                tetromino.shape = new_shape; // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
            } else {
                println!("Collision detected doing some calculations");
                // Adjust position if collision detected
                tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                tetromino.shape = new_shape; // Rotate the shape
                adjust_position(&mut tetromino);
                commands.entity(entity).insert(NeedsRedraw {});
            }
            lock_in_timer.0.reset(); // Reset the lock-in timer when moving right 
        }
    }
}

pub fn gravity(
    mut commands: Commands,
    time: Res<Time>,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (entity, mut tetromino) in tetromino.iter_mut() {
            if !is_tetromino_hit_floor(&tetromino) {
                    tetromino.position.1 -= 1;
                    // Add NeedsRedraw component to tetromino to trigger redraw
                    commands.entity(entity).insert(NeedsRedraw {});
                }
        }
    }
}

pub fn detect_lock_position(
    mut lock_in_timer: ResMut<LockInTimer>,
    time: Res<Time>,
    tetromino_query: Query<&Tetromino, With<Active>>,
) {
    for tetromino in tetromino_query.iter() {
        if is_tetromino_hit_floor(&tetromino){
            lock_in_timer.0.tick(time.delta());
        }
    }
}

fn is_collision(position: &(i32, i32), shape: &[[bool; 4]; 4]) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if shape[y][x] {
                let new_x = position.0 + x as i32;
                let new_y = position.1 - y as i32;
                if new_x >= GRID_WIDTH as i32 || new_x < 0 || new_y < 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn adjust_position(tetromino: &mut Tetromino) {
    // Adjust position to prevent collision with left wall
    while is_tetromino_hit_left_wall(tetromino) {
        tetromino.position.0 += 1;
    }

    // Adjust position to prevent collision with right wall
    while is_tetromino_hit_right_wall(tetromino) {
        tetromino.position.0 -= 1;
    }

    // Adjust position to prevent collision with floor
    while is_tetromino_hit_floor(tetromino) {
        tetromino.position.1 += 1;
    }
}


// Helpers
fn is_tetromino_hit_floor(tetromino: &Tetromino) -> bool{
    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                if tetromino.position.1 - y as i32 <= 0 {
                    return true;
                }
            }
        }
    }
    return false;
} 

fn is_tetromino_hit_left_wall(tetromino: &Tetromino) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                if tetromino.position.0 + x as i32 <= 0 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_tetromino_hit_right_wall(tetromino: &Tetromino) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                if tetromino.position.0 + x as i32 >= GRID_WIDTH as i32 - 1 {
                    return true;
                }
            }
        }
    }
    return false;
}