use bevy::prelude::*;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const GRID_CELL_SIZE: f32 = 40.0;
pub const CELL_BORDER_WIDTH: f32 = 2.0;

// Grid resource to store the state of each cell 
#[derive(Resource)]
pub struct Grid{
    pub cells: Vec<CellState>, 
}
impl Grid {
    pub fn new() -> Self {
        let cells = vec![CellState::Empty; GRID_WIDTH * GRID_HEIGHT];
        Grid { cells }
    }
}

#[derive(Component)]
pub struct GridCell;

#[derive(Resource)]
pub struct GridConfig {
    pub start_x: f32,
    pub start_y: f32,
}

#[derive(Resource, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Filled(Color),
}

#[derive(Event)]
pub struct RedrawGridEvent;

pub fn draw_grid(mut commands: Commands, grid: Res<Grid>, grid_config: Res<GridConfig>, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            let color = match &grid.cells[index] {
                CellState::Empty => Color::srgb(0.0, 0.0, 0.0),
                CellState::Filled(color) => *color 
            };

            let cell_x = grid_config.start_x + x as f32 * GRID_CELL_SIZE;
            let cell_y = grid_config.start_y + y as f32 * GRID_CELL_SIZE;

            // Draw the cell
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::default())),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(cell_x, cell_y, -20.0)
                    .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0)),
                GridCell {},
            ));
            
        }
    }
}

pub fn redraw_grid(
    mut commands: Commands,
    mut redraw_grid_events: EventReader<RedrawGridEvent>,
    grid: Res<Grid>,
    grid_config: Res<GridConfig>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    grid_cell_query: Query<(Entity, &GridCell)>,
) {
    // Draw the grid again
    if !redraw_grid_events.is_empty() {
        redraw_grid_events.clear();
        // Clear the previous grid cells
        for (entity, _) in grid_cell_query.iter() {
            commands.entity(entity).despawn();
        }

        draw_grid(commands, grid, grid_config, materials, meshes);
    }
}

// Helpers
pub fn get_vec_index_from_grid_coordinates(x: i32, y: i32) -> usize {
    (y * GRID_WIDTH as i32 + x) as usize
}