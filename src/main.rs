use bevy::{prelude::*, window::{PrimaryWindow, WindowResolution}};
use player::systems::{spawn_player, player_movement, shoot_laser, laser_movement};

mod player;

fn main() {
    App::new()

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
        .add_systems(Update, (player_movement, shoot_laser, laser_movement))
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

