use bevy::{prelude::*, render::render_resource::encase::private::Length};

use crate::game_manager::GameRestartEvent;
use crate::scoring::{RedrawLevelAndScoreEvent, Scoring, calculate_score};

pub struct GridPlugin;
impl Plugin for GridPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(Grid::new())
            .insert_resource(GridConfig {
                start_x: -(GRID_WIDTH as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
                start_y: -(GRID_HEIGHT as f32 * (GRID_CELL_SIZE + CELL_BORDER_WIDTH)) / 2.0,
            })
            .add_event::<RedrawGridEvent>()
            .add_event::<CheckForLinesEvent>()
            .add_systems(Startup, draw_grid)
            .add_systems(Update, (check_for_lines, redraw_grid, check_for_lines, reset_grid));
    }
}

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20; 
pub const GRID_HIDDEN_HEIGHT: usize = 6; // Every row above 20 is hidden
pub const GRID_CELL_SIZE: f32 = 40.0;
pub const CELL_BORDER_WIDTH: f32 = 2.0;

// Grid resource to store the state of each cell 
#[derive(Resource)]
pub struct Grid{
    pub cells: Vec<CellState>, 
}
impl Grid {
    pub fn new() -> Self {
        let cells = vec![CellState::Empty; GRID_WIDTH * (GRID_HEIGHT + GRID_HIDDEN_HEIGHT)];
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

#[derive(Resource, Clone, PartialEq, Debug, Copy)]
pub enum CellState {
    Empty,
    Filled(Color),
}

#[derive(Event)]
pub struct RedrawGridEvent;

pub fn draw_grid(
    mut commands: Commands,
    grid: Res<Grid>, 
    grid_config: Res<GridConfig>, 
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>
) {
    for y in 0..GRID_HEIGHT + GRID_HIDDEN_HEIGHT {
        for x in 0..GRID_WIDTH {
            let index = y * GRID_WIDTH + x;
            let color = match &grid.cells[index] {
                CellState::Empty => Color::srgb(0.12, 0.12, 0.18),
                CellState::Filled(color) => *color 
            };

            let cell_x = grid_config.start_x + x as f32 * GRID_CELL_SIZE;
            let cell_y = grid_config.start_y + y as f32 * GRID_CELL_SIZE;

            // Don't draw the hidden cells
            if y < GRID_HEIGHT {
                // Draw the cell
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(cell_x, cell_y, -69.0)
                        .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0)),
                    GridCell {},
                ));
            }
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

pub fn reset_grid(
    mut redraw_grid_event: EventWriter<RedrawGridEvent>,
    mut game_restart_event: EventReader<GameRestartEvent>,
    mut grid_resource: ResMut<Grid>
){
    if !game_restart_event.is_empty() {
        game_restart_event.clear();
        // Change the grid to be all empty by replacing the resource
        for cell in grid_resource.cells.iter_mut(){
            if *cell != CellState::Empty{
                *cell = CellState::Empty;
            }
        }
        // Send redraw grid event
        redraw_grid_event.send(RedrawGridEvent);
    }
}

// Checking for lines
#[derive(Event)]
pub struct CheckForLinesEvent;

pub fn check_for_lines(
    mut grid: ResMut<Grid>,
    mut check_for_lines_event: EventReader<CheckForLinesEvent>,
    mut redraw_grid_event: EventWriter<RedrawGridEvent>,
    mut scoring_resource: ResMut<Scoring>,
    mut redraw_level_and_score_event: EventWriter<RedrawLevelAndScoreEvent>,
) {
    // Figure out if any or some lines have been achieved on a 1D vector of CellStates 
    if !check_for_lines_event.is_empty(){
        check_for_lines_event.clear();
        let mut index_of_rows_filled: Vec<(usize, usize)> = vec![];
        let mut slice_start = 0;
        let mut slice_end = 10;
        for _ in 0..grid.cells.length() / 10{
            let row: &[CellState] = &grid.cells[slice_start..slice_end];

            if is_all_row_filled(row){
                index_of_rows_filled.push((slice_start, slice_end));
            }
            slice_start += 10;
            slice_end += 10;
        }

        // If there is a row that has been filled, drain them reversal style
        if !index_of_rows_filled.is_empty() {

            let mut lines_just_cleared= 0;

            for row in index_of_rows_filled.iter().rev(){
                grid.cells.drain(row.0..row.1);
                lines_just_cleared += 1;
            }

            // Increase lines, calculate score and send redraw event 
            scoring_resource.lines_cleared += lines_just_cleared; 
            scoring_resource.score += calculate_score(lines_just_cleared, scoring_resource.level);
            redraw_level_and_score_event.send(RedrawLevelAndScoreEvent); 

            for _ in 0..index_of_rows_filled.len() * 10{
                grid.cells.push(CellState::Empty);
            }
            redraw_grid_event.send(RedrawGridEvent);
        }
    }
}


// Helpers
fn is_all_row_filled(cells: &[CellState]) -> bool {
    return cells.iter().all(|cell| matches!(cell, CellState::Filled(_)));
}

pub fn get_vec_index_from_grid_coordinates(x: i32, y: i32) -> usize {
    (y * GRID_WIDTH as i32 + x) as usize
}