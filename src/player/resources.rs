use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerParams {
    pub health: f32,
    pub lives: i32
}

impl Default for PlayerParams {
    fn default() -> PlayerParams {
        PlayerParams { health: 100.0, lives: 3 }
    }
}