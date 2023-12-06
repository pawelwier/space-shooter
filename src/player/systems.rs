use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::explosion::systems::spawn_explosion;
use crate::object::{
    MovingObject,
    enemy::components::Enemy,
    meteor::{
        systems::spawn_small_meteors,
        components::{
            Meteor,
            MeteorSize
        },
    },
    powerup::components::PowerUp
};
use super::components::{
    Laser,
    Player,
    PlayerMovement
};
use super::{
    PLAYER_SPEED,
    PLAYER_WIDTH,
    PLAYER_HEIGHT,
    LASER_SPEED,
    LASER_HEIGHT
};
use super::helpers::{
    key_just_pressed,
    key_pressed,
    key_just_released
};

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
            Player {
                movement: PlayerMovement::None
            }
        )
    );
}

fn handle_player_turn(
    keyboard_input: Res<Input<KeyCode>>,
    transform: &mut Transform
) {
    if key_just_pressed(&keyboard_input, KeyCode::Left) 
        || key_just_released(&keyboard_input, KeyCode::Right) { 
            transform.rotate_y(-15.0);
        }
    if key_just_pressed(&keyboard_input, KeyCode::Right) 
        || key_just_released(&keyboard_input, KeyCode::Left) {
            transform.rotate_y(15.0);
        }
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
        let (mut x, /*mut*/ y) = (0.0, 0.0);

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

        handle_player_turn(keyboard_input, &mut transform);
        
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
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut laser_query: Query<(&mut Transform, Entity), With<Laser>>,
    time: Res<Time>
) {
    let window = window_query.get_single().unwrap();

    for (mut transform, entity) in laser_query.iter_mut() {
        let direction = Vec3::new(0.0, 1.0, 0.0);
        transform.translation += direction * LASER_SPEED * time.delta_seconds();

        if transform.translation.y >= window.height() {
            commands.entity(entity).despawn();
        }
    } 
}

pub fn laser_hit_meteor(
    mut commands: Commands,
    mut laser_query: Query<(&Transform, Entity), With<Laser>>,
    mut meteor_query: Query<(&Transform, &Meteor, Entity, &MovingObject), With<Meteor>>,
    asset_server: Res<AssetServer>,
) {
    for (laser_transform, laser_entity) in laser_query.iter_mut() {
        for (meteor_transform, meteor, meteor_entity, moving_object) in meteor_query.iter_mut() {
            // TODO: Fix measure distance
            if laser_transform.translation.distance(meteor_transform.translation) < moving_object.size.1 / 2.0 + LASER_HEIGHT / 2.0 {
                commands.entity(laser_entity).despawn();
                commands.entity(meteor_entity).despawn();
                spawn_explosion(
                    &mut commands,
                    (laser_transform.translation.x, laser_transform.translation.y),
                    &asset_server
                );
                if meteor.size == MeteorSize::Large {
                    spawn_small_meteors(
                        &mut commands,
                        (laser_transform.translation.x, laser_transform.translation.y),
                        &asset_server
                    );
                }
            }
        }
    }
}

fn hit_player_damage(
    commands: &mut Commands,
    player_entity: Entity
) {
    commands.entity(player_entity).despawn();
}

pub fn object_hit_player(
    mut commands: Commands,
    object_query: Query<(&Transform, &MovingObject, Option<&Meteor>, Option<&Enemy>, Option<&PowerUp>), With<MovingObject>>,
    player_query: Query<(&Transform, Entity), With<Player>>
) {
    for (player_transform, player_entity) in player_query.iter() {
        for object in object_query.iter() {
            let (
                transform,
                moving_object,
                meteor_option,
                enemy_option,
                power_up_option
            ) = object;
            let distance = player_transform.translation.distance(transform.translation);
            // TODO: get more precise distance using rects
            if distance < (moving_object.size.0 + moving_object.size.1 / 2.0) / 2.0 + PLAYER_HEIGHT / 2.0 {
                if meteor_option.is_some() || enemy_option.is_some() {
                    hit_player_damage(
                        &mut commands,
                        player_entity
                    )
                }
                if power_up_option.is_some() {
                    match Some(power_up_option) {
                        Some(power_up) => {
                            println!("type: {:?}", power_up);
                        },
                        None => {}
                    }
                }
            }

        }
    }
}