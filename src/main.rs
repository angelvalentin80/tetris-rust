use std::cell;

use bevy::{ecs::storage::Resources, prelude::*, state::commands};

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, print_grid).chain())
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Adding our grid resource
    commands.insert_resource(Grid::new());
}

// Grid resource to store the state of each cell 
#[derive(Resource)]
struct Grid{
    cells: Vec<CellState>, 
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

fn draw_grid(mut commands: Commands, grid: Res<Grid>, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    const GRID_CELL_SIZE: f32 = 40.0;
    let cell_border_width = 2.0;
    let grid_start_position_x = -(GRID_WIDTH as f32 * (GRID_CELL_SIZE + cell_border_width)) / 2.0;
    let grid_start_position_y = GRID_HEIGHT as f32 * (GRID_CELL_SIZE + cell_border_width) / 2.0;

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            let color = match grid.cells[index] {
                CellState::Empty => Color::srgb(0.0, 0.0, 0.0),
                CellState::Filled => Color::srgb(0.0, 0.0, 1.0),
            };

            let cell_x = grid_start_position_x + x as f32 * GRID_CELL_SIZE;
            let cell_y = grid_start_position_y - y as f32 * GRID_CELL_SIZE;

            // Draw the cell
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(cell_x, cell_y, 0.0)
                    .with_scale(Vec3::new(GRID_CELL_SIZE - cell_border_width, GRID_CELL_SIZE - cell_border_width, 1.0))
            ));
            
        }
    }
}
