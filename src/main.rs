use bevy::{prelude::*, window::{PrimaryWindow, WindowResolution}};
use object::meteor::{systems::{meteor_movement, spawn_meteors_over_time, tick_meteor_spawn_timer}, resources::MeteorSpawnTimer};
use player::systems::{spawn_player, player_movement, shoot_laser, laser_movement, laser_hit_meteor};

pub mod player;
pub mod object;

fn main() {
    App::new()
    .   init_resource::<MeteorSpawnTimer>()
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
            tick_meteor_spawn_timer, player_movement, shoot_laser, 
            laser_movement, meteor_movement, spawn_meteors_over_time, laser_hit_meteor
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