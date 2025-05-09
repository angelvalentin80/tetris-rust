use bevy::prelude::*;
use crate::grid::{GridConfig, GRID_WIDTH, GRID_CELL_SIZE, GRID_HEIGHT};
use crate::game_manager::GameStartEvent;

pub struct ScoringPlugin;
impl Plugin for ScoringPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(Scoring{level: 1, score: 0, lines_cleared: 0})
            .add_event::<RedrawLevelAndScoreEvent>()
            .add_event::<LevelUpEvent>()
            .add_systems(Update, (draw_level_and_score, reset_level_and_score));
    }
}

#[derive(Resource)]
pub struct Scoring{
    pub level: usize,
    pub score: usize,
    pub lines_cleared: usize 
}

#[derive(Component)]
pub struct ScoringText {}

#[derive(Event)]
pub struct RedrawLevelAndScoreEvent;

#[derive(Event)]
pub struct LevelUpEvent;

pub fn draw_level_and_score(
    mut commands: Commands,
    mut scoring_resource: ResMut<Scoring>,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
    mut redraw_level_and_score_event: EventReader<RedrawLevelAndScoreEvent>,
    mut game_start_event: EventReader<GameStartEvent>,
    scoring_text_query: Query<(Entity, &ScoringText)>,
    mut level_up_event: EventWriter<LevelUpEvent> 
){
    if !redraw_level_and_score_event.is_empty() || !game_start_event.is_empty(){
        redraw_level_and_score_event.clear();
        game_start_event.clear();

        for (entity, _) in scoring_text_query.iter(){
            commands.entity(entity).despawn();
        }

        // Calculate level
        let old_level = scoring_resource.level;
        scoring_resource.level = calculate_level(&scoring_resource.lines_cleared);
        if old_level < scoring_resource.level{
            level_up_event.send(LevelUpEvent);
        }

        let font = asset_server.load("fonts/gg-sans-Regular.ttf");
        let text_font = TextFont {
            font: font.clone(),
            font_size: 25.0,
            ..default()
        };
        let text_color = TextColor(Color::srgb(0.8, 0.85, 0.9));

        // Draw Level
        let text_x = (grid_config.start_x + (GRID_WIDTH as f32 * GRID_CELL_SIZE)) + 100.0;
        let text_y = grid_config.start_y + ((GRID_HEIGHT as f32 / 2.0) * GRID_CELL_SIZE);

        commands.spawn((
            Text2d::new(format!("Level\n{}", scoring_resource.level)),
            text_font.clone(),
            text_color,
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(text_x, text_y, 0.0),
            ScoringText {}
        ));

        // Draw Score 
        let text_x = (grid_config.start_x + (GRID_WIDTH as f32 * GRID_CELL_SIZE)) + 100.0;
        let text_y = grid_config.start_y + ((GRID_HEIGHT as f32 / 2.0) * GRID_CELL_SIZE) - 75.0;

        commands.spawn((
            Text2d::new(format!("Score\n{}", scoring_resource.score)),
            text_font.clone(),
            text_color,
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(text_x, text_y, 0.0),
            ScoringText {}
        ));

        // Total Lines Cleared
        let text_x = (grid_config.start_x + (GRID_WIDTH as f32 * GRID_CELL_SIZE)) + 100.0;
        let text_y = grid_config.start_y + ((GRID_HEIGHT as f32 / 2.0) * GRID_CELL_SIZE) - 150.0;

        commands.spawn((
            Text2d::new(format!("Lines Cleared\n{}", scoring_resource.lines_cleared)),
            text_font.clone(),
            text_color,
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(text_x, text_y, 0.0),
            ScoringText {}
        ));


    }
}

pub fn calculate_level(total_lines_cleared: &usize) -> usize {
    (total_lines_cleared / 10) + 1
}

pub fn calculate_score(lines_cleared_at_once: usize, level: usize) -> usize {
    match lines_cleared_at_once {
        1 => 100 * level,
        2 => 300 * level,
        3 => 500 * level,
        4 => 800 * level,
        _ => 0 // Should never happen
    }
}

pub fn reset_level_and_score(
    mut game_start_event: EventReader<GameStartEvent>,
    mut redraw_level_and_score_event: EventWriter<RedrawLevelAndScoreEvent>,
    mut scoring_resource: ResMut<Scoring>
){
    // Receive Game Start Event and reset score
    if !game_start_event.is_empty(){
        game_start_event.clear();

        // Reset score 
        scoring_resource.level = 0;
        scoring_resource.score = 0;
        scoring_resource.lines_cleared = 0;

        // Send event to redraw the level and score 
        redraw_level_and_score_event.send(RedrawLevelAndScoreEvent);
    }

}