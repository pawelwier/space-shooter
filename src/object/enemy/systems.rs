use bevy::{prelude::*, window::PrimaryWindow};

use crate::object::{Object, MovingObject};

use super::{components::Enemy, resources::EnemySpawnTimer};

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let enemy = Enemy { 
        direction: 0.0
    };

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform
                {
                    translation: Vec3::new(Enemy::get_spawn_x(&enemy, window.width()), window.height(), 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::PI),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/enemy_ship_red.png"),
                ..default()
            },
            enemy,
            MovingObject {}
        )
    );
}

// TODO: add as shared fn among Objects
pub fn enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, Entity, &mut Enemy), With<Enemy>>,
    time: Res<Time>
) {
    for (mut transform, entity, enemy) in enemy_query.iter_mut() {
        // TODO: maybe random y
        let direction = Vec3::new(enemy.direction, -1.0, 0.0);
        transform.translation += direction * Enemy::speed(&enemy) * time.delta_seconds();

        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    } 
}

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_enemies_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>
) {
    if enemy_spawn_timer.timer.finished() {
        spawn_enemy(commands, window_query, asset_server);
    }
}
