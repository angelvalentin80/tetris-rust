use::bevy::prelude::*;

use crate::resources::GameState;

#[derive(Event)]
pub struct GameStartEvent;

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