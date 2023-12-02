use bevy::{prelude::*, window::PrimaryWindow};

use super::components::{Laser, Player};
use super::{PLAYER_SPEED, PLAYER_WIDTH, PLAYER_HEIGHT, LASER_SPEED};
use super::helpers::{key_just_pressed, key_pressed};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, PLAYER_HEIGHT, 0.0),
                texture: asset_server.load("sprites/player_orange.png"),
                ..default()
            },
            Player {}
        )
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    // TODO change to iter to allow multiplayer
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let mut direction = Vec3::ZERO;
        let (mut x, mut y) = (0.0, 0.0);

        // TODO: keep y for vertical movement

        if key_pressed(&keyboard_input, KeyCode::Left) { x = -1.0; }
        if key_pressed(&keyboard_input, KeyCode::Right) { x = 1.0; }
        // if key_pressed(&keyboard_input, KeyCode::Up) { y = 1.0; }
        // if key_pressed(&keyboard_input, KeyCode::Down) { y = -1.0; }

        let border_left = transform.translation.x <= (PLAYER_WIDTH / 2.0) && x == -1.0;
        let border_right = transform.translation.x >= window.width() - PLAYER_WIDTH / 2.0 && x == 1.0;
        if  border_left || border_right { return; }

        if x != 0.0 || y != 0.0 {
            direction += Vec3::new(x, y, 0.0);
            direction = direction.normalize();
        }
        
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();        
    }
}

pub fn shoot_laser(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if let Ok(player_transform) = player_query.get_single() {
        if key_just_pressed(&keyboard_input, KeyCode::Space) {    
            let x = player_transform.translation.x;
            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform::from_xyz(x, PLAYER_HEIGHT * 2.0, 0.0),
                        texture: asset_server.load("sprites/laser_green.png"),
                        ..default()
                    },
                    Laser {}
                )
            );
        }
    }
}

pub fn laser_movement(
    mut laser_query: Query<&mut Transform, With<Laser>>,
    time: Res<Time>
) {
    for mut transform in laser_query.iter_mut() {
        let direction = Vec3::new(0.0, 1.0, 0.0);
        transform.translation += direction * LASER_SPEED * time.delta_seconds()
    } 
}