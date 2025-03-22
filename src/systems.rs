use bevy::prelude::*;
use crate::tetromino::{Tetromino, Active, NeedsRedraw};
use crate::resources::GravityTimer;

pub fn gravity(
    mut commands: Commands,
    time: Res<Time>,
    mut tetromino: Query<(Entity, &mut Tetromino), With<Active>>,
    mut gravity_timer: ResMut<GravityTimer>,
) {
    gravity_timer.0.tick(time.delta());
    if gravity_timer.0.just_finished() {
        for (entity, mut tetromino) in tetromino.iter_mut() {

        if tetromino.position.1 > tetromino.get_shape_height() - 1{
                tetromino.position.1 -= 1;
                // Add NeedsRedraw component to tetromino to trigger redraw
                commands.entity(entity).insert(NeedsRedraw {});
            }
        }
    }
}