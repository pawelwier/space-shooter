use bevy::prelude::*;

#[derive(Resource)]
pub struct ScoreResource {
    pub points: f32
}

impl Default for ScoreResource {
    fn default() -> ScoreResource {
        ScoreResource { points: 0.0 }
    }
}