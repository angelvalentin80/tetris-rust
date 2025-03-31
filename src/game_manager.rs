use::bevy::prelude::*;

use crate::grid::{GridConfig, GRID_CELL_SIZE, GRID_WIDTH, GRID_HEIGHT};

pub struct GameManagerPlugin;
impl Plugin for GameManagerPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(GameState { started: false })
            .add_event::<GameStartEvent>()
            .add_event::<GameRestartEvent>()
            .add_event::<GameLoseEvent>()
            .add_systems(Update, (detect_start_game, detect_restart_game, spawn_lose_text, animate_lose_text, reset_lose_text));
    }
}


#[derive(Event)]
pub struct GameStartEvent;

#[derive(Event)]
pub struct GameLoseEvent;

#[derive(Resource)]
pub struct GameState {
    pub started: bool,
}

pub fn detect_start_game(
    mut game_start_event: EventWriter<GameStartEvent>,
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    // Detect if I press enter key
    if !game_state.started {
        if keyboard_input.just_pressed(KeyCode::Enter) {
            // Send the game start event
            game_start_event.send(GameStartEvent);
            game_state.started = true;
        }
    }
}

#[derive(Event)]
pub struct GameRestartEvent;

pub fn detect_restart_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut game_restart_event: EventWriter<GameRestartEvent>,
    mut game_lose_event: EventReader<GameLoseEvent>
){
    // Send GameRestartEvent
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        game_state.started = false;
        game_restart_event.send(GameRestartEvent);
    }
    // This will be getting triggered when we detect game is lost
    if !game_lose_event.is_empty() {
        game_lose_event.clear();
        game_state.started = false;
        game_restart_event.send(GameRestartEvent);
    }
}

#[derive(Component)]
pub struct AnimateLoseText;

pub fn spawn_lose_text(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut game_lose_event: EventReader<GameLoseEvent>
){
    if !game_lose_event.is_empty() {
        game_lose_event.clear();

        let text_font = TextFont {
            font_size: 100.0,
            ..default()
        };

        // Draw Lose text in center of grid 
        let text_x = grid_config.start_x + ((GRID_WIDTH as f32 / 2.0) * GRID_CELL_SIZE);
        let text_y = grid_config.start_y + ((GRID_HEIGHT as f32 / 2.0) * GRID_CELL_SIZE);

        commands.spawn((
            Text2d::new("You Lose"),
            TextColor(Color::srgb(1.0, 0.2, 0.2)),
            text_font.clone(),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(text_x, text_y, 0.0),
            AnimateLoseText {}
        ));
    }
}

pub fn animate_lose_text(
    mut lose_text_query: Query<&mut Transform, With<AnimateLoseText>>,
    time: Res<Time>
){
    for mut transform in &mut lose_text_query {
        let scale = ops::sin(time.elapsed_secs()) * 0.1 + 1.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
} 

pub fn reset_lose_text(
    mut commands: Commands,
    mut game_start_event: EventReader<GameStartEvent>,
    lose_text_query: Query<(Entity, &mut Transform), With<AnimateLoseText>>,
){
    if !game_start_event.is_empty(){
        game_start_event.clear();
        for (entity, _) in lose_text_query.iter(){
            commands.entity(entity).despawn();
        }
    }
}