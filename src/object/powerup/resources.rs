use bevy::prelude::*;

pub const POWER_UP_SPAWN_TIME: f32 = 3.5;

#[derive(Resource)]
pub struct PowerUpSpawnTimer {
    pub timer: Timer
}

impl Default for PowerUpSpawnTimer {
    fn default() -> PowerUpSpawnTimer {
        PowerUpSpawnTimer { timer: Timer::from_seconds(POWER_UP_SPAWN_TIME, TimerMode::Repeating) }
    }
}