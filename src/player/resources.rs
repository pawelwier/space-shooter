use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerParams {
    pub health: f32,
    pub can_flash: bool
}

impl Default for PlayerParams {
    fn default() -> PlayerParams {
        PlayerParams { 
            health: 100.0,
            can_flash: false
        }
    }
}