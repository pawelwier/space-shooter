use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        WindowResolution
    }
};
use explosion::systems::despawn_explosions_on_timeout;
use game::{
    resources::ScoreResource,
    systems::{
        spawn_score,
        update_health, 
        spawn_health_bar_init, 
        spawn_flash_icon
    }, 
    events::{
        HealthChange,
        Flash
    }, 
    WINDOW_HEIGHT,
    WINDOW_WIDTH
};
use object::{
    meteor::{
        systems::{
            meteor_movement,
            spawn_meteors_over_time,
            tick_meteor_spawn_timer
        },
        resources::MeteorSpawnTimer
    }, 
    enemy::{
        systems::{
            tick_enemy_spawn_timer,
            spawn_enemies_over_time,
            enemy_movement, shoot_enemy_laser, enemy_laser_movement
        }, 
        resources::{
            EnemySpawnTimer,
            EnemyLaserTimers
        }
    }, 
    powerup::{
        resources::PowerUpSpawnTimer, 
        systems::{
            tick_power_up_spawn_timer,
            spawn_power_ups_over_time,
            power_up_movement
        }
    }
};
use player::{
    systems::{
        spawn_player,
        player_movement,
        shoot_laser,
        laser_movement,
        laser_hit_object,
        object_hit_player, flash
    }, 
    resources::PlayerParams
};

pub mod player;
pub mod object;
pub mod explosion;
pub mod game;

fn main() {
    // TODO: turn into plugins
    App::new()
        .init_resource::<MeteorSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<PowerUpSpawnTimer>()
        .init_resource::<ScoreResource>()
        .init_resource::<PlayerParams>()
        .init_resource::<EnemyLaserTimers>()
        .add_event::<HealthChange>()
        .add_event::<Flash>()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SPACE SHOOTER".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_systems(Startup, (
            spawn_camera, spawn_player, spawn_score, spawn_health_bar_init
        ))
        .add_systems(Update, (
            tick_meteor_spawn_timer, player_movement, shoot_laser, object_hit_player,
            tick_enemy_spawn_timer, spawn_enemies_over_time, enemy_movement,
            tick_power_up_spawn_timer, spawn_power_ups_over_time, power_up_movement,
            laser_movement, meteor_movement, spawn_meteors_over_time, laser_hit_object, 
            despawn_explosions_on_timeout, update_health, flash, spawn_flash_icon, shoot_enemy_laser,
            enemy_laser_movement
        ))
        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        }
    );
}