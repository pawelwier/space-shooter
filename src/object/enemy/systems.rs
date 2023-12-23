use bevy::{
    prelude::*, 
    window::PrimaryWindow
};

use crate::object::{
    SpawnedEntity,
    MovingObject, enemy::{components::EnemyLaser, ENEMY_LASER_SPEED, ENEMY_LASER_WIDTH, ENEMY_LASER_HEIGHT, ENEMY_LASER_DAMAGE}
};

use super::{
    components::Enemy, 
    resources::{
        EnemySpawnTimer,
        EnemyLaserTimers
    }, 
    ENEMY_SHIP_SPEED,
    ENEMY_SHIP_WIDTH,
    ENEMY_SHIP_HEIGHT, 
    ENEMY_SHIP_DAMAGE,
    ENEMY_LASER_FREQUENCY
};

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_laser_timers: &mut ResMut<EnemyLaserTimers>
) {
    let window = window_query.get_single().unwrap();
    let enemy = Enemy {};

    let enemy_entity = commands.spawn(
        (
            SpriteBundle {
                transform: Transform
                {
                    translation: Vec3::new(Enemy::get_spawn_x(&enemy, window.width()), window.height(), 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/enemy_ship_red.png"),
                ..Default::default()
            },
            enemy,
            MovingObject {
                speed: ENEMY_SHIP_SPEED,
                direction: 0.0,
                size: (ENEMY_SHIP_WIDTH, ENEMY_SHIP_HEIGHT),
                hps: ENEMY_SHIP_DAMAGE
            }
        )
    ).id();

    enemy_laser_timers.timers.push(
        (
            enemy_entity, 
            Timer::from_seconds(ENEMY_LASER_FREQUENCY, TimerMode::Repeating)
        )
    );
}

// TODO: add as shared fn among Objects
pub fn enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, Entity, &MovingObject), With<Enemy>>,
    time: Res<Time>
) {
    for (mut transform, entity, object) in enemy_query.iter_mut() {
        // TODO: maybe random y
        let direction = Vec3::new(object.direction, -1.0, 0.0);
        transform.translation += direction * object.speed * time.delta_seconds();

        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    } 
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    mut enemy_laser_timers: ResMut<EnemyLaserTimers>,
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());

    for (_, timer) in enemy_laser_timers.timers.iter_mut() {
        timer.tick(time.delta());
    }
}


pub fn spawn_enemies_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    mut enemy_laser_timers: ResMut<EnemyLaserTimers>

) {
    if enemy_spawn_timer.timer.finished() {
        spawn_enemy(commands, window_query, asset_server, &mut enemy_laser_timers);
    }
}

pub fn shoot_enemy_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    enemy_laser_timers: Res<EnemyLaserTimers>
) {
    for (enemy_entity, timer) in enemy_laser_timers.timers.iter() {
        if timer.finished() {
            if let Ok((entity, transform)) = enemy_query.get(*enemy_entity) {
                if enemy_entity.index() == entity.index() {
                    let start_vector = transform.translation.xy();

                    println!("{:?}:, {:?}", start_vector.x, start_vector.y);
                    commands.spawn(
                        (
                            SpriteBundle {
                                transform: Transform
                                {
                                    translation: Vec3::new(start_vector.x, start_vector.y, 0.0),
                                    // TODO: rotate and direction towards Player
                                    // rotation: Quat::from_rotation_z(), 
                                    ..Default::default()
                                },
                                texture: asset_server.load("sprites/laser_red.png"),
                                ..Default::default()
                            },
                            EnemyLaser {},
                            MovingObject {
                                speed: ENEMY_LASER_SPEED,
                                direction: 0.0,
                                size: (ENEMY_LASER_WIDTH, ENEMY_LASER_HEIGHT),
                                hps: ENEMY_LASER_DAMAGE
                            }
                        )
                    );
                }
            }
        }
    }
}

pub fn enemy_laser_movement(
    mut commands: Commands,
    mut enemy_laser_query: Query<(&mut Transform, Entity, &MovingObject), With<EnemyLaser>>,
    time: Res<Time>
) {
    for (mut transform, entity, object) in enemy_laser_query.iter_mut() {
        // TODO: point at Player
        let direction = Vec3::new(object.direction, -1.0, 0.0);
        transform.translation += direction * object.speed * time.delta_seconds();

        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    } 
}