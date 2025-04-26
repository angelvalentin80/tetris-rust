use bevy::prelude::*;
use crate::grid::{GridConfig, GRID_HEIGHT, GRID_CELL_SIZE, GRID_HIDDEN_HEIGHT};
 
pub struct TipsPlugin;
impl Plugin for TipsPlugin{
    fn build(&self, app: &mut App){
        app
            .add_event::<DrawGameTipsEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, (toggle_game_tips, draw_game_tips));
    }
}

fn setup(
    mut draw_game_tips_event: EventWriter<DrawGameTipsEvent>
){
    // Adding our help text
    draw_game_tips_event.send(DrawGameTipsEvent);
}

#[derive(Component)]
pub struct GameTipText;

#[derive(Event)]
pub struct DrawGameTipsEvent;

pub fn draw_game_tips(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
    mut draw_game_tips_event: EventReader<DrawGameTipsEvent>,
){
    if !draw_game_tips_event.is_empty(){
        draw_game_tips_event.clear();

        let font = asset_server.load("fonts/gg-sans-Regular.ttf");
        let text_font = TextFont {
            font: font.clone(),
            font_size: 13.0,
            ..default()
        };
        let text_color = TextColor(Color::srgb(0.8, 0.85, 0.9));

        let help_texts= vec![
            "ENTER to start game",
            "Left/Right Arrow to move",
            "Down Arrow to drop",
            "Up Arrow to rotate clockwise",
            "CTRL to rotate counter clockwise",
            "SPACE to hard drop",
            "R to reset",
            "H to hide this text"
            ];

        let text_x = grid_config.start_x - 150.0;
        let mut text_y = grid_config.start_y + (GRID_HEIGHT - GRID_HIDDEN_HEIGHT) as f32 * GRID_CELL_SIZE;
        let text_gap = 50.0;

        for text in help_texts{
            commands.spawn((
                Text2d::new(text),
                text_font.clone(),
                text_color,
                TextLayout::new_with_justify(JustifyText::Left),
                Transform::from_xyz(text_x, text_y, 0.0),
                GameTipText{}
            ));

            text_y -= text_gap;
        }
    }
}

pub fn toggle_game_tips(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_tip_text_query: Query<Entity, With<GameTipText>>,
    mut draw_game_tips_event: EventWriter<DrawGameTipsEvent>
){
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        // draw game tips
        if game_tip_text_query.is_empty(){
            draw_game_tips_event.send(DrawGameTipsEvent);
        } else {
            for entity in game_tip_text_query.iter(){
                commands.entity(entity).despawn();
            }
        }
    }
}