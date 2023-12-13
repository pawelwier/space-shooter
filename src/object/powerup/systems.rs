use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::object::{
    SpawnedEntity,
    MovingObject
};

use super::{
    components::{
        PowerUp,
        PowerUpType,
        get_power_up_sprite_path,
        get_random_power_type
    }, 
    POWER_UP_SPEED,
    POWER_UP_WIDTH,
    POWER_UP_HEIGHT,
    resources::PowerUpSpawnTimer
};

pub fn spawn_power_up(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    power_type: PowerUpType
) {
    let window = window_query.get_single().unwrap();
    let sprite_path = get_power_up_sprite_path(&power_type.clone());
    let power_up = PowerUp {
        power_type
    };

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform
                {
                    translation: Vec3::new(PowerUp::get_spawn_x(&power_up, window.width()), window.height(), 0.0),
                    ..Default::default()
                },
                texture: asset_server.load(sprite_path),
                ..Default::default()
            },
            power_up,
            MovingObject {
                speed: POWER_UP_SPEED,
                direction: 0.0,
                size: (POWER_UP_WIDTH, POWER_UP_HEIGHT),
                damage: 0.0 // TODO: gain hps when colliding with some power ups?
            }
        )
    );
}

// TODO: add as shared fn among Objects
pub fn power_up_movement(
    mut commands: Commands,
    mut power_up_query: Query<(&mut Transform, Entity, &MovingObject), With<PowerUp>>,
    time: Res<Time>
) {
    for (mut transform, entity, object) in power_up_query.iter_mut() {
        // TODO: maybe random y
        let direction = Vec3::new(object.direction, -1.0, 0.0);
        transform.translation += direction * object.speed * time.delta_seconds();

        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    } 
}

pub fn tick_power_up_spawn_timer(
    mut power_up_spawn_timer: ResMut<PowerUpSpawnTimer>,
    time: Res<Time>
) {
    power_up_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_power_ups_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    power_up_spawn_timer: Res<PowerUpSpawnTimer>
) {
    if power_up_spawn_timer.timer.finished() {
        spawn_power_up(commands, window_query, asset_server, get_random_power_type());
    }
}
