use std::cell;

use bevy::{ecs::storage::Resources, prelude::*, state::commands};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;
const GRID_CELL_SIZE: f32 = 40.0;
const CELL_BORDER_WIDTH: f32 = 2.0;

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
    }, Active {},
    ));
    commands.spawn(Tetromino {
        letter: TetrominoLetter::J,
        shape: [[true, false, false, false], 
                [true, true, true, false], 
                [false, false, false, false], 
                [false, false, false, false]],
        position: spawn_position,
        rotation: 0,
        color: TetrominoColor::DarkBlue.to_color(),
    });
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

// Grid resource to store the state of each cell 
#[derive(Resource)]
struct Grid{
    cells: Vec<CellState>, 
}

#[derive(Resource)]
struct GridConfig {
    start_x: f32,
    start_y: f32,
}

impl Grid {
    fn new() -> Self {
        let cells = vec![CellState::Empty; GRID_WIDTH * GRID_HEIGHT];
        Grid { cells }
    }
}

#[derive(Resource, Clone)]
enum CellState {
    Empty,
    Filled, // TODO add more states for different colors / tetrominoes
}

fn print_grid(grid: Res<Grid>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            match grid.cells[index] {
                CellState::Empty => print!("."),
                CellState::Filled => print!("X"),
            }
        }
        println!();
    }
}

fn draw_grid(mut commands: Commands, grid: Res<Grid>, grid_config: Res<GridConfig>, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            let color = match grid.cells[index] {
                CellState::Empty => Color::srgb(0.0, 0.0, 0.0),
                CellState::Filled => Color::srgb(0.0, 0.0, 1.0),
            };

            let cell_x = grid_config.start_x + x as f32 * GRID_CELL_SIZE;
            let cell_y = grid_config.start_y + y as f32 * GRID_CELL_SIZE;

            // Draw the cell
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(cell_x, cell_y, -20.0)
                    .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0))
            ));
            
        }
    }
}

// Tetrominoes
enum TetrominoLetter {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}

enum TetrominoColor {
    LightBlue,
    DarkBlue,
    Orange,
    Yellow,
    Green,
    Red,
    Magenta,
}
impl TetrominoColor {
    fn to_color(&self) -> Color {
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
struct Active {}

#[derive(Component)]
struct Tetromino {
    letter: TetrominoLetter,
    shape: [[bool; 4]; 4], // 4x4 grid for the tetromino shape
    position: (usize, usize), // (x, y) position on the grid
    rotation: usize, // 0-3 for 0-270 degrees
    color: Color,
}
#[derive(Component)]
struct TetrominoCell {}

#[derive(Resource)]
struct GravityTimer(Timer);

fn draw_tetromino(
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

fn gravity(
    time: Res<Time>,
    mut tetromino_cell_query: Query<(&mut TetrominoCell, &mut Transform)>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (_, mut transform) in tetromino_cell_query.iter_mut() {
            // Move the tetromino down by one cell each second
            transform.translation.y -= GRID_CELL_SIZE;
        }
    }
}