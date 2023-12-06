use bevy::{
    prelude::*,
    window::{
        PrimaryWindow,
        WindowResolution
    }
};
use explosion::systems::despawn_explosions_on_timeout;
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
            enemy_movement
        }, 
        resources::EnemySpawnTimer
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
use player::systems::{
    spawn_player,
    player_movement,
    shoot_laser,
    laser_movement,
    laser_hit_meteor,
    object_hit_player
};

pub mod player;
pub mod object;
pub mod explosion;

fn main() {
    App::new()
        .init_resource::<MeteorSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<PowerUpSpawnTimer>()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SPACE SHOOTER".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(1000.0, 1000.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, (
            tick_meteor_spawn_timer, player_movement, shoot_laser, object_hit_player,
            tick_enemy_spawn_timer, spawn_enemies_over_time, enemy_movement,
            tick_power_up_spawn_timer, spawn_power_ups_over_time, power_up_movement,
            laser_movement, meteor_movement, spawn_meteors_over_time, laser_hit_meteor, despawn_explosions_on_timeout
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
            ..default()
        }
    );
}