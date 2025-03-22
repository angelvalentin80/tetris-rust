use::bevy::prelude::*;

#[derive(Resource)]
pub struct GravityTimer(pub Timer);

#[derive(Resource)]
pub struct LockInTimer(pub Timer);