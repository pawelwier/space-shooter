use bevy::prelude::*;

pub const ENEMY_SPAWN_TIME: f32 = 2.5;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

#[derive(Resource)]
pub struct EnemyLaserTimers {
    pub timers: Vec<(Entity, Timer)>
}

impl Default for EnemyLaserTimers {
    fn default() -> EnemyLaserTimers {
        EnemyLaserTimers { timers: vec![] }
    }
}   