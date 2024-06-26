use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::explosion::systems::spawn_explosion;
use crate::game::AppState;
use crate::game::events::{HealthChange, Flash};
use crate::game::systems::add_points_to_score;
use crate::game::{
    components::ScoreComponent,
    resources::ScoreResource
};
use crate::object::enemy::ENEMY_SHIP_POINTS;
use crate::object::meteor::{
    METEOR_LARGE_POINTS,
    METEOR_SMALL_POINTS
};
use crate::object::powerup::systems::toggle_can_flash;
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
    powerup::components::{
        PowerUp,
        PowerUpType
    }
};
use super::components::{
    Laser,
    Player,
    PlayerMovement
};
use super::resources::PlayerParams;
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
                ..Default::default()
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
        // TODO: keep y for vertical movement
        let (mut x, /*mut*/ y) = (0.0, 0.0);

        let border_left = transform.translation.x <= (PLAYER_WIDTH / 2.0);
        let border_right = transform.translation.x >= window.width() - PLAYER_WIDTH / 2.0;

        if key_pressed(&keyboard_input, KeyCode::Left) && !border_left { x = -1.0; }
        if key_pressed(&keyboard_input, KeyCode::Right) && !border_right { x = 1.0; }
        // if key_pressed(&keyboard_input, KeyCode::Up) { y = 1.0; }
        // if key_pressed(&keyboard_input, KeyCode::Down) { y = -1.0; }

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
                        ..Default::default()
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

pub fn flash(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut object_query: Query<(&Transform, Entity), (With<MovingObject>, Without<PowerUp>)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut params: ResMut<PlayerParams>,
    mut flash_event_writer: EventWriter<Flash>
) {
    if key_just_pressed(&keyboard_input, KeyCode::ControlLeft) && params.can_flash {
        toggle_can_flash(&mut params, false);

        flash_event_writer.send(Flash { display: false });

        for (transform, entity) in object_query.iter_mut() {
            commands.entity(entity).despawn();

            spawn_explosion(
                &mut commands,
                (transform.translation.x, transform.translation.y),
                &asset_server
            );
        }
    }
}

pub fn laser_hit_object(
    mut commands: Commands,
    mut laser_query: Query<(&Transform, Entity), With<Laser>>,
    mut object_query: Query<(
        &Transform, Option<&Meteor>, Option<&Enemy>, Entity, &MovingObject
    ),  Or<(With<Meteor>, With<Enemy>)>>,
    asset_server: Res<AssetServer>,
    mut score_resource: ResMut<ScoreResource>,
    mut score_query: Query<&mut Text, With<ScoreComponent>>,
) {
    for (laser_transform, laser_entity) in laser_query.iter_mut() {
        for (meteor_transform, meteor_option, enemy_option, object_entity, moving_object) in object_query.iter_mut() {
            // TODO: Fix measure distance
            if laser_transform.translation.distance(meteor_transform.translation) < moving_object.size.1 / 2.0 + LASER_HEIGHT / 2.0 {
                commands.entity(laser_entity).despawn();
                commands.entity(object_entity).despawn();
                
                spawn_explosion(
                    &mut commands,
                    (laser_transform.translation.x, laser_transform.translation.y),
                    &asset_server
                );
                
                // TODO: refactor Some, match, is_some()
                if meteor_option.is_some() {
                    match Some(meteor_option) {
                        Some(meteor) => {
                            let is_large = meteor.unwrap().size == MeteorSize::Large;
                            let score = if is_large { METEOR_LARGE_POINTS } else { METEOR_SMALL_POINTS };
                            add_points_to_score(score, &mut score_resource, &mut score_query);
        
                            if is_large {
                                spawn_small_meteors(
                                    &mut commands,
                                    (laser_transform.translation.x, laser_transform.translation.y),
                                    &asset_server
                                );
                            }
                        }
                        None => {}
                    }   
                }

                if enemy_option.is_some() {
                    add_points_to_score(ENEMY_SHIP_POINTS, &mut score_resource, &mut score_query);
                }
            }
        }
    }
}

fn hit_player_damage(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_entity: Entity,
    health_resource: &mut ResMut<PlayerParams>,
    hps: f32,
    location: (f32, f32),
    health_change_event_writer: &mut EventWriter<HealthChange>,
    app_state_next_state: &mut ResMut<NextState<AppState>>
) {
    let is_max = (health_resource.health + hps) >= 100.0;
    health_resource.health += if is_max { 100.0 - health_resource.health } else { hps };

    health_change_event_writer.send(HealthChange {});
    if health_resource.health <= 0.0 {
        despawn_player(commands, player_entity);
        spawn_explosion(
            commands,
            (location.0, location.1),
            &asset_server
        );
        app_state_next_state.set(AppState::GameOver);
    }
}

fn despawn_player(
    commands: &mut Commands,
    player_entity: Entity
) {
    commands.entity(player_entity).despawn();
}

pub fn object_hit_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    object_query: Query<(
        Entity, &Transform, &MovingObject, Option<&PowerUp>
    ), With<MovingObject>>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    mut score_resource: ResMut<ScoreResource>,
    mut score_query: Query<&mut Text, With<ScoreComponent>>,
    mut player_params: ResMut<PlayerParams>,
    mut health_change_event_writer: EventWriter<HealthChange>,
    mut flash_event_writer: EventWriter<Flash>,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    for (player_transform, player_entity) in player_query.iter() {
        for object in object_query.iter() {
            let (
                object_entity,
                object_transform,
                object,
                power_up_option
            ) = object;
            let distance = player_transform.translation.distance(object_transform.translation);
            // TODO: get more precise distance using rects

            // TODO: refactor Some, match, is_some()
            if distance < (object.size.0 + object.size.1 / 2.0) / 2.0 + PLAYER_HEIGHT / 2.0 {
                commands.entity(object_entity).despawn();
                hit_player_damage(
                    &mut commands,
                    &asset_server,
                    player_entity,
                    &mut player_params,
                    object.hps,
                    (player_transform.translation.x, player_transform.translation.y),
                    &mut health_change_event_writer,
                    &mut app_state_next_state
                );

                if power_up_option.is_some() {
                    match Some(power_up_option) {
                        Some(power_up) => {
                            let power_type = &power_up.unwrap().power_type;
                            let score: f32;

                            match power_type {
                                // TODO: Add special powers
                                PowerUpType::Bolt => {
                                    toggle_can_flash(&mut player_params, true);
                                    flash_event_writer.send(Flash { display: true });

                                    score = 10.0;
                                }
                                PowerUpType::Shield => {
                                    score = 25.0;
                                }
                                PowerUpType::Star => {
                                    score = 40.0;
                                }
                            }
                            add_points_to_score(score, &mut score_resource, &mut score_query);
                        },
                        None => {}
                    }
                }
            }

        }
    }
}