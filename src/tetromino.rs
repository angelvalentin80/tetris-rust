use std::time::Duration;

use bevy::prelude::*;

use crate::game_manager::{GameRestartEvent, GameStartEvent, GameLoseEvent};
use crate::grid::{get_vec_index_from_grid_coordinates, CellState, Grid, GridConfig, CELL_BORDER_WIDTH, GRID_CELL_SIZE, GRID_HEIGHT, GRID_HIDDEN_HEIGHT, GRID_WIDTH, RedrawGridEvent, CheckForLinesEvent};
use crate::queue::TetrominoQueue;
use crate::scoring::{Scoring, LevelUpEvent};

pub struct TetrominoPlugin;
impl Plugin for TetrominoPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(GravityTimer(Timer::from_seconds(gravity_seconds_for_level(1), TimerMode::Repeating)))
            .insert_resource(LockInTimer(Timer::from_seconds(0.5, TimerMode::Once)))
            .add_event::<SpawnTetrominoEvent>()
            .add_event::<RedrawGhostCellsEvent>()
            .add_event::<LockInTetrominoEvent>()
            .add_event::<SpawnNextPieceEvent>()
            .add_systems(Update, (spawn_tetromino, draw_tetromino, draw_ghost_piece, draw_next_piece_text, spawn_next_piece, draw_next_piece).chain()) 
            .add_systems(Update, (gravity, detect_lock_position, lock_in_tetromino, move_tetromino, update_gravity_timer, maybe_lock_in_tetromino, despawn_active_tetromino, despawn_next_piece, reset_lock_in_timer, reset_gravity_timer));
    }
}

// Components
#[derive(Component, Clone)]
pub struct Tetromino {
    pub shape: [[bool; 4]; 4], // 4x4 grid for the tetromino shape
    pub position: (i32, i32), // (x, y) position on the grid
    pub rotation: usize, // 0-3 for 0-270 degrees
    pub color: Color,
    pub letter: TetrominoLetter,
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
            TetrominoLetter::I => TetrominoColor::Cyan.to_color(),
            TetrominoLetter::J => TetrominoColor::Indigo.to_color(),
            TetrominoLetter::L => TetrominoColor::Tangerine.to_color(),
            TetrominoLetter::O => TetrominoColor::Gold.to_color(),
            TetrominoLetter::S => TetrominoColor::Emerald.to_color(),
            TetrominoLetter::Z => TetrominoColor::Crimson.to_color(),
            TetrominoLetter::T => TetrominoColor::Orchid.to_color(),
        };
        Self {
            shape,
            position: (3 , 21), // Spawn position
            rotation: 0,
            color,
            letter
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
}

#[derive(Component)]
pub struct TetrominoCell {}

#[derive(Component)]
pub struct Active {}

#[derive(Component)]
pub struct NeedsRedraw();

#[derive(Component)]
pub struct GhostCell {}

#[derive(Component)]
pub struct NextPiece;

#[derive(Component)]
pub struct NextPieceCells;

#[derive(Component)]
pub struct NextTetrominoPieceText;

// Enums
#[derive(Clone, Debug, PartialEq, Copy)]
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
    Cyan,
    Indigo,
    Tangerine,
    Gold,
    Emerald,
    Crimson,
    Orchid
}
impl TetrominoColor {
    pub fn to_color(&self) -> Color {
        match self {
            TetrominoColor::Cyan => Color::srgb(0.0, 0.95, 1.0),
            TetrominoColor::Indigo => Color::srgb(0.1, 0.3, 0.8),
            TetrominoColor::Tangerine => Color::srgb(1.0, 0.6, 0.2),
            TetrominoColor::Gold => Color::srgb(1.0, 0.85, 0.2),
            TetrominoColor::Emerald => Color::srgb(0.0, 0.8, 0.4),
            TetrominoColor::Crimson => Color::srgb(0.85, 0.1, 0.3),
            TetrominoColor::Orchid => Color::srgb(0.7, 0.3, 0.8),
        }
    }
}

// Events
#[derive(Event)]
pub struct SpawnTetrominoEvent;

#[derive(Event)]
pub struct LockInTetrominoEvent;

#[derive(Event)]
pub struct SpawnNextPieceEvent;

#[derive(Event)]
pub struct RedrawGhostCellsEvent;

//Resources
#[derive(Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Resource)]
pub struct LockInTimer(pub Timer);

pub fn spawn_tetromino(
    mut commands: Commands,
    mut tetromino_queue: ResMut<TetrominoQueue>,
    mut spawn_tetromino_event: EventReader<SpawnTetrominoEvent>,
    mut spawn_next_piece_event: EventWriter<SpawnNextPieceEvent> 
) {
    if !spawn_tetromino_event.is_empty() {
        spawn_tetromino_event.clear();

        commands.spawn((
            Tetromino::create_tetromino(tetromino_queue.queue.pop_front().unwrap()),
            Active {},
            NeedsRedraw {}
        ));
        
        // We spawn next piece here so that this happens after the current tetromino is popped 
        // from the queue so we don't end up having the same piece being the "Tetromino" and the
        // NextPiece 
        spawn_next_piece_event.send(SpawnNextPieceEvent);
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

pub fn despawn_active_tetromino(
    mut commands: Commands,
    mut game_restart_event: EventReader<GameRestartEvent>,
    tetromino_query: Query<(Entity, &Tetromino), With<Active>>,
    tetromino_cell_query: Query<(Entity, &TetrominoCell)>,
){
    // Despawn the active tetromino, and the tetromino cells
    if !game_restart_event.is_empty(){
        game_restart_event.clear();
        for (entity, _) in tetromino_query.iter(){
            commands.entity(entity).despawn();
        }
        for (entity, _) in tetromino_cell_query.iter(){
            commands.entity(entity).despawn();
        }
    }
}

pub fn move_tetromino(
    mut commands: Commands,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    mut lock_in_timer: ResMut<LockInTimer>,
    mut gravity_timer: ResMut<GravityTimer>,
    grid: Res<Grid>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut redraw_ghost_cells_event: EventWriter<RedrawGhostCellsEvent>,
) {
    for (entity, mut tetromino) in tetromino.iter_mut() {

        // Move Left
        if !is_tetromino_hit_left_wall(&tetromino) && !is_tetromino_hit_left_piece(&tetromino, &grid) {
            if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                tetromino.position.0 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
                redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
                lock_in_timer.0.reset(); // Reset the lock-in timer when moving left
            } 
        }

        // Move Right 
        if !is_tetromino_hit_right_wall(&tetromino) && !is_tetromino_hit_right_piece(&tetromino, &grid) {
            if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                tetromino.position.0 += 1;
                commands.entity(entity).insert(NeedsRedraw {});
                redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
                lock_in_timer.0.reset(); // Reset the lock-in timer when moving right 
            } 
        }

        // Move Down 
        if !is_tetromino_hit_floor(&tetromino) && !is_tetromino_hit_floor_piece(&tetromino, &grid) {
            if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                tetromino.position.1 -= 1;
                commands.entity(entity).insert(NeedsRedraw {});
                gravity_timer.0.reset();
                lock_in_timer.0.reset(); // Reset the lock-in timer when moving right 
            }
        }

        // Rotate Clockwise
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && tetromino.letter != TetrominoLetter::O {
            let new_shape = tetromino.rotate_tetromino_shape_clockwise();
            if !is_collision(&tetromino.position, &new_shape, &grid) {
                // If new shape has no collision, rotate normally 
                tetromino.rotation = (tetromino.rotation + 1) % 4; // Rotate the tetromino
                tetromino.shape = new_shape; // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
                redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
            } else {
                // Adjust position using SRS 
                let from_rotation = &tetromino.rotation;
                let to_rotation = (&tetromino.rotation + 1) % 4;
                let kick_table = get_kick_table_scenario(&tetromino.letter, &from_rotation, &to_rotation);
                let maybe_new_kick=  maybe_try_kicks(&tetromino, &kick_table, &grid, &new_shape);
                if let Some((dx, dy)) = maybe_new_kick {
                    tetromino.position.0 += dx;
                    tetromino.position.1 -= dy;
                    tetromino.rotation = (tetromino.rotation + 1) % 4; // Rotate the tetromino clockwise
                    tetromino.shape = new_shape; // Rotate the shape
                    commands.entity(entity).insert(NeedsRedraw {});
                    redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
                } else {
                    // If no valid kick found, do nothing
                    return;
                }
            }
            lock_in_timer.0.reset(); // Reset the lock-in timer when rotating 
        }

        // Rotate Counter Clockwise 
        if keyboard_input.just_pressed(KeyCode::ControlLeft) && tetromino.letter != TetrominoLetter::O {
            let new_shape = tetromino.rotate_tetromino_shape_counter_clockwise();
            if !is_collision(&tetromino.position, &new_shape, &grid) {
                // If new shape has no collision, rotate normally 
                tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                tetromino.shape = new_shape; // Rotate the shape
                commands.entity(entity).insert(NeedsRedraw {});
                redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
            } else {
                // Adjust position using SRS 
                let from_rotation = &tetromino.rotation;
                let to_rotation = (&tetromino.rotation + 3) % 4;
                let kick_table = get_kick_table_scenario(&tetromino.letter, &from_rotation, &to_rotation);
                let maybe_new_kick=  maybe_try_kicks(&tetromino, &kick_table, &grid, &new_shape);

                if let Some((dx, dy)) = maybe_new_kick {
                    tetromino.position.0 += dx;
                    tetromino.position.1 -= dy;
                    tetromino.rotation = (tetromino.rotation + 3) % 4; // Rotate the tetromino counter-clockwise
                    tetromino.shape = new_shape; // Rotate the shape
                    commands.entity(entity).insert(NeedsRedraw {});
                    redraw_ghost_cells_event.send(RedrawGhostCellsEvent);
                } else {
                    // If no valid kick found, do nothing
                    return;
                }
            }
            lock_in_timer.0.reset(); // Reset the lock-in timer when rotating
        }

        // Hard Drop
        if keyboard_input.just_pressed(KeyCode::Space) {
            while !is_tetromino_hit_floor(&tetromino) && !is_tetromino_hit_floor_piece(&tetromino, &grid) {
                tetromino.position.1 -= 1;
            }
            commands.entity(entity).insert(NeedsRedraw {});
            lock_in_timer.0.reset(); // Reset the lock-in timer when hard dropping
            gravity_timer.0.reset();
        }
    }
}

pub fn gravity(
    mut commands: Commands,
    time: Res<Time>,
    grid: Res<Grid>,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    mut gravity_timer: ResMut<GravityTimer>
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (entity, mut tetromino) in tetromino.iter_mut() {
            if !is_tetromino_hit_floor(&tetromino) && !is_tetromino_hit_floor_piece(&tetromino, &grid) {
                    tetromino.position.1 -= 1;
                    // Add NeedsRedraw component to tetromino to trigger redraw
                    commands.entity(entity).insert(NeedsRedraw {});
                }
        }
    }
}

pub fn gravity_seconds_for_level(level: usize) -> f32 {
    // What the gravity timer will be for each level
    match level {
        1 => 1.0, // OG 1.0
        2 => 0.8,
        3 => 0.6,
        4 => 0.4,
        5 => 0.3,
        6 => 0.25,
        7 => 0.2,
        8 => 0.15,
        9 => 0.1,
        10 => 0.05,
        _ => 0.01667, // Max gravity
    }
}

pub fn update_gravity_timer(
    mut gravity_timer: ResMut<GravityTimer>,
    scoring_resource: Res<Scoring>,
    mut level_up_event: EventReader<LevelUpEvent> 
){
    // Listens for level up event, then changes the time duration
    if !level_up_event.is_empty(){
        level_up_event.clear();
        let new_duration = gravity_seconds_for_level(scoring_resource.level);
        gravity_timer.0.set_duration(Duration::from_secs_f32(new_duration));
    }
}

pub fn reset_gravity_timer(
    mut game_restart_event: EventReader<GameRestartEvent>,
    mut gravity_timer: ResMut<GravityTimer>
){
    if !game_restart_event.is_empty(){
        game_restart_event.clear();
        gravity_timer.0.reset();
    }
}

pub fn detect_lock_position(
    mut lock_in_timer: ResMut<LockInTimer>,
    time: Res<Time>,
    grid: Res<Grid>,
    tetromino_query: Query<&Tetromino, With<Active>>
) {
    for tetromino in tetromino_query.iter() {
        if is_tetromino_hit_floor(&tetromino) || is_tetromino_hit_floor_piece(&tetromino, &grid) {
            lock_in_timer.0.tick(time.delta());
        }
    }
}

pub fn maybe_lock_in_tetromino(
    lock_in_timer: Res<LockInTimer>,
    mut lock_in_tetromino_event: EventWriter<LockInTetrominoEvent>,
){
    if lock_in_timer.0.finished(){
        lock_in_tetromino_event.send(LockInTetrominoEvent);
    }
}

pub fn lock_in_tetromino(
    mut commands: Commands,
    mut grid: ResMut<Grid>, 
    mut redraw_grid_event: EventWriter<RedrawGridEvent>,
    mut spawn_tetromino_event: EventWriter<SpawnTetrominoEvent>,
    tetromino_query: Query<(Entity, &Tetromino), With<Active>>,
    tetromino_cell_query: Query<(Entity, &TetrominoCell)>,
    ghost_cell_query: Query<(Entity, &GhostCell)>,
    mut redraw_ghost_cells_event: EventReader<RedrawGhostCellsEvent>, 
    mut check_for_lines_event: EventWriter<CheckForLinesEvent>,
    mut lock_in_tetromino_event: EventReader<LockInTetrominoEvent>,
    mut game_lose_event: EventWriter<GameLoseEvent>,
    mut lock_in_timer: ResMut<LockInTimer>
) {
    if !lock_in_tetromino_event.is_empty(){
        lock_in_tetromino_event.clear();
        

        for (entity, tetromino) in tetromino_query.iter() {

            // Check whether the tetromino piece is in a "losing" condition
            if is_lose_conditions(&tetromino, &grid) {
                game_lose_event.send(GameLoseEvent);
            }

            // Lock in the tetromino by updating the grid state
            let start_x = tetromino.position.0;
            let start_y = tetromino.position.1;

            for y in 0..4 {
                for x in 0..4 {
                    if tetromino.shape[y][x] {
                        let index = get_vec_index_from_grid_coordinates(start_x + x as i32, start_y - y as i32);
                        grid.cells[index] = CellState::Filled(tetromino.color);
                    }
                }
            }

            commands.entity(entity).remove::<Active>();
            commands.entity(entity).despawn();
        }

        for (entity, _) in tetromino_cell_query.iter() {
            commands.entity(entity).despawn();
        }

        if !redraw_ghost_cells_event.is_empty() {
            redraw_ghost_cells_event.clear();
            for (entity, _) in ghost_cell_query.iter() {
                commands.entity(entity).despawn();
            }
        }

        check_for_lines_event.send(CheckForLinesEvent);
        redraw_grid_event.send(RedrawGridEvent);
        spawn_tetromino_event.send(SpawnTetrominoEvent);
        lock_in_timer.0.reset()
    } 
}

pub fn reset_lock_in_timer(
    mut game_restart_event: EventReader<GameRestartEvent>,
    mut lock_in_timer: ResMut<LockInTimer>

){
    if !game_restart_event.is_empty(){
        game_restart_event.clear();
        lock_in_timer.0.reset();
    }
}

fn is_collision(
    position: &(i32, i32),
    shape: &[[bool; 4]; 4],
    grid: &Grid
) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if shape[y][x] {
                let new_x = position.0 + x as i32;
                let new_y = position.1 - y as i32;
                if new_x >= GRID_WIDTH as i32 || new_x < 0 || new_y < 0 || new_y >= GRID_HEIGHT as i32 + GRID_HIDDEN_HEIGHT as i32 {
                    return true;
                } 
                let index = get_vec_index_from_grid_coordinates(new_x, new_y);
                if grid.cells[index] != CellState::Empty {
                    return true;
                }
            }
        }
    }
    false
}

// Helpers 
// Grid wall collisions
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

// Grid collisions with other locked in pieces
fn is_tetromino_hit_floor_piece(
    tetromino: &Tetromino,
    grid: &Grid
) -> bool {
    let start_x = tetromino.position.0;
    let start_y = tetromino.position.1;

    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                let new_x = start_x + x as i32;
                let new_y = start_y - y as i32;
                // Calculating the cell below the tetromino to see if it's filled or not
                let index = get_vec_index_from_grid_coordinates(new_x, new_y - 1);
                if new_y > 0 && grid.cells[index] != CellState::Empty {
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_tetromino_hit_left_piece(
    tetromino: &Tetromino,
    grid: &Grid
) -> bool {
    let start_x = tetromino.position.0;
    let start_y = tetromino.position.1;

    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                let new_x = start_x + x as i32;
                let new_y = start_y - y as i32;
                // Calculating the cell to the left of the tetromino to see if it's filled or not
                let index = get_vec_index_from_grid_coordinates(new_x as i32 - 1, new_y as i32);
                if new_x > 0 && grid.cells[index] != CellState::Empty {
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_tetromino_hit_right_piece(
    tetromino: &Tetromino,
    grid: &Grid
) -> bool {
    let start_x = tetromino.position.0;
    let start_y = tetromino.position.1;

    for y in 0..4 {
        for x in 0..4 {
            if tetromino.shape[y][x] {
                let new_x = start_x + x as i32;
                // Calculating the cell to the right of the tetromino to see if it's filled or not
                let index = get_vec_index_from_grid_coordinates(new_x as i32 + 1, start_y - y as i32);
                if new_x < GRID_WIDTH as i32 - 1 && grid.cells[index] != CellState::Empty {
                    return true;
                }
            }
        }
    }
    return false;

}

// SRS
fn get_kick_table_scenario(
    letter: &TetrominoLetter,
    from: &usize,
    to: &usize
) -> Vec<(i32, i32)> {
    // These kick tables assume normal tetris where Y increases as you go down
    // BUT we are using a coordinate system where Y decreases as you go down 
    // So these kick tables are correct, but when we use them we have to invert 
    match letter {
        TetrominoLetter::J | TetrominoLetter::L | TetrominoLetter::S | TetrominoLetter::T | TetrominoLetter::Z => {
            // JLSTZ pieces
            match (from, to) {
                (0, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (1, 2) => vec![(0,0), (1,0), (1,1), (0,-2), (1,-2)],
                (2, 3) => vec![(0,0), (1,0), (1,-1), (0,2), (1,2)],
                (3, 0) => vec![(0,0), (-1,0), (-1,-1), (0,2), (-1,2)],
                (1, 0) => vec![(0,0), (1,0), (1,-1), (0,2), (1,2)],
                (2, 1) => vec![(0,0), (-1,0), (-1,-1), (0,2), (-1,2)],
                (3, 2) => vec![(0,0), (-1,0), (-1,1), (0,-2), (-1,-2)],
                (0, 3) => vec![(0,0), (1,0), (1,1), (0,-2), (1,-2)],
                _ => vec![],
            }
        }
        TetrominoLetter::I => {
            // I piece
            match (from, to) {
                (0, 1) => vec![(0,0), (-2,0), (1,0), (-2,-1), (1,2)],
                (1, 2) => vec![(0,0), (-1,0), (2,0), (-1,2), (2,-1)],
                (2, 3) => vec![(0,0), (2,0), (-1,0), (2,1), (-1,-2)],
                (3, 0) => vec![(0,0), (1,0), (-2,0), (1,-2), (-2,1)],
                (1, 0) => vec![(0,0), (2,0), (-1,0), (2,-1), (-1,2)],
                (2, 1) => vec![(0,0), (1,0), (-2,0), (1,2), (-2,-1)],
                (3, 2) => vec![(0,0), (-2,0), (1,0), (-2,-1), (1,2)],
                (0, 3) => vec![(0,0), (-1,0), (2,0), (-1,2), (2,-1)],
                _ => vec![],
            }
        }
        TetrominoLetter::O => {
            // O piece
            // No kick table needed
            vec![]
        }
    }
}

fn maybe_try_kicks(
    tetromino: &Tetromino,
    kick_table: &Vec<(i32, i32)>,
    grid: &Grid,
    shape: &[[bool; 4]; 4],
) -> Option<(i32, i32)> {
    // Rotate your local 4x4 grid.
    // You apply each offset from the kick table in order:
    // For each, check: does this new position collide or go out of bounds?
    // If yes, try next kick
    // If no, accept offset
    let current_position_x = tetromino.position.0;
    let current_position_y = tetromino.position.1;

    for (dx, dy) in kick_table.iter() {
        // Inverting the Y value of the kick table to match our coordinate system
        let real_dy = -dy;
        let new_x = current_position_x + dx;
        let new_y = current_position_y - real_dy;
        // Is there a collision on the walls?
        if is_collision(&(new_x, new_y), shape, grid) {
            continue; // Out of bounds
        }
        // If we reach here, we have a valid position
        return Some((*dx, real_dy));
    }
    None
}

// Ghost Piece



pub fn draw_ghost_piece(
    mut commands: Commands,
    tetromino: Query<&Tetromino, With<Active>>,
    ghost_cells_query: Query<(Entity, &GhostCell)>,
    grid: Res<Grid>,
    grid_config: Res<GridConfig>,
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>
){
    if !ghost_cells_query.is_empty() {
        for (entity, _) in ghost_cells_query.iter() {
            commands.entity(entity).despawn();
        }
    }

    for tetromino in tetromino.iter() {
        let mut ghost_tetromino = tetromino.clone();
        while !is_tetromino_hit_floor(&ghost_tetromino) && !is_tetromino_hit_floor_piece(&ghost_tetromino, &grid) {
            ghost_tetromino.position.1 -= 1;
        }
        for y in 0..4 {
            for x in 0..4 {
                if ghost_tetromino.shape[y][x] {
                    let cell_x = grid_config.start_x + (ghost_tetromino.position.0 + x as i32) as f32 * GRID_CELL_SIZE;
                    let cell_y = grid_config.start_y + (ghost_tetromino.position.1 - y as i32) as f32 * GRID_CELL_SIZE;

                    // Draw the cells
                    if !is_tetromino_hit_floor(tetromino) && !is_tetromino_hit_floor_piece(tetromino, &grid) {
                        commands.spawn((
                            Mesh2d(meshes.add(Rectangle::default())),
                            MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.2))), // Make the ghost piece transparent
                            Transform::from_xyz(cell_x, cell_y, 0.0)
                                .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0)),
                            GhostCell {},
                        ));
                    }
                }
            }
        }
    }

}

// Next Tetromino Piece
pub fn draw_next_piece_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
    mut game_start_event: EventReader<GameStartEvent>
){
    if !game_start_event.is_empty(){
        game_start_event.clear();

        let font = asset_server.load("fonts/gg-sans-Regular.ttf");
        let text_font = TextFont {
            font: font.clone(),
            font_size: 25.0,
            ..default()
        };
        let text_color = TextColor(Color::srgb(0.8, 0.85, 0.9));

        let text_x = (grid_config.start_x + (GRID_WIDTH as f32 * GRID_CELL_SIZE)) + 100.0;
        let text_y = grid_config.start_y + (GRID_HEIGHT as f32 * GRID_CELL_SIZE) - 25.0;

        commands.spawn((
            Text2d::new("Next Piece"),
            text_color,
            text_font.clone(),
            TextLayout::new_with_justify(JustifyText::Right),
            Transform::from_xyz(text_x, text_y, 0.0),
            NextTetrominoPieceText {}
        ));
    }
} 


pub fn spawn_next_piece(
    mut commands: Commands,
    tetromino_queue: Res<TetrominoQueue>,
    mut game_start_event: EventReader<GameStartEvent>,
    next_piece_query: Query<(Entity, &NextPiece)>,
    next_piece_cell_query: Query<(Entity, &NextPieceCells)>,
    mut spawn_next_piece_event: EventReader<SpawnNextPieceEvent>
){
    if !game_start_event.is_empty() || !spawn_next_piece_event.is_empty(){

        for (entity, _) in next_piece_query.iter(){
            commands.entity(entity).despawn();
        }
        for (entity, _) in next_piece_cell_query.iter(){
            commands.entity(entity).despawn()
        }

        game_start_event.clear();
        spawn_next_piece_event.clear();

        if let Some(&upcoming_piece) = tetromino_queue.queue.front() {
            // Spawn "NextPiece" Entity
            commands.spawn((
                Tetromino::create_tetromino(upcoming_piece),
                NextPiece {},
                NeedsRedraw{}
            ));
        };
    }
}
pub fn draw_next_piece(
    mut commands: Commands,
    next_piece_tetromino_query: Query<(Entity, &Tetromino), (With<NextPiece>, With<NeedsRedraw>)>,
    mut materials: ResMut<Assets<ColorMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    grid_config: Res<GridConfig>,
){
    for (entity, next_piece) in next_piece_tetromino_query.iter(){
        // Draw new entities
        let initial_x = (grid_config.start_x + (GRID_WIDTH as f32 * GRID_CELL_SIZE)) + 50.0;
        let initial_y = grid_config.start_y + (GRID_HEIGHT as f32 * GRID_CELL_SIZE) - 100.0;
        for y in 0..4 {
            for x in 0..4 {
                if next_piece.shape[y][x] {
                    let cell_x = initial_x + (x as f32 * GRID_CELL_SIZE);
                    let cell_y = initial_y - (y as f32 * GRID_CELL_SIZE);

                    // Draw the next piece 
                    commands.spawn((
                        Mesh2d(meshes.add(Rectangle::default())),
                        MeshMaterial2d(materials.add(next_piece.color)),
                        Transform::from_xyz(cell_x, cell_y, 0.0)
                            .with_scale(Vec3::new(GRID_CELL_SIZE - CELL_BORDER_WIDTH, GRID_CELL_SIZE - CELL_BORDER_WIDTH, 1.0)),
                        NextPieceCells{}
                    ));
                }
            }
        }
        commands.entity(entity).remove::<NeedsRedraw>(); // Remove the NeedsRedraw component after drawing 
    }
}

pub fn despawn_next_piece(
    mut commands: Commands, 
    mut game_restart_event: EventReader<GameRestartEvent>,
    next_piece_query: Query<(Entity, &NextPiece)>,
    next_piece_cells_query: Query<(Entity, &NextPieceCells)>,

){
    // Despawn the next piece and its cells
    if !game_restart_event.is_empty(){
        game_restart_event.clear();

        for (entity, _) in next_piece_query.iter(){
            commands.entity(entity).despawn();
        }
        for (entity, _) in next_piece_cells_query.iter(){
            commands.entity(entity).despawn();
        }
    }
}

// Lose Conditions
pub fn is_lose_conditions(
    tetromino: &Tetromino,
    grid_resource: &Grid
) -> bool {
    // Lose Condition 1
    // When a new piece appears at the top, but part of it 
    //immediately overlaps with blocks already on the board → Game Over.
    // Lose Condition 2
    // If a piece lands and locks into place, but part of it is outside 
    // the visible play area, the game ends.
    for y in 0..4{ //TODO this pattern is around a lot we can probably make this into a function
        for x in 0..4 {
            if tetromino.shape[y][x]{
                let shape_y = tetromino.position.1 - y as i32;                     
                if is_tetromino_hit_floor_piece(&tetromino, &grid_resource) && shape_y > 19 {
                    return true
                }
            } 
        }
    }
    false
} 