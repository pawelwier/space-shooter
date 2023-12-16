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
        Meteor,
        MeteorSize
    }, 
    resources::MeteorSpawnTimer, 
    METEOR_SMALL_SPEED, 
    METEOR_LARGE_SPEED, 
    METEOR_LARGE_WIDTH, 
    METEOR_LARGE_HEIGHT, 
    METEOR_SMALL_WIDTH, 
    METEOR_SMALL_HEIGHT, METEOR_LARGE_DAMAGE, METEOR_SMALL_DAMAGE
};

pub fn spawn_meteor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let meteor = Meteor { 
        size: MeteorSize::Large
    };

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(Meteor::get_spawn_x(&meteor, window.width()), window.height(), 0.0),
                texture: asset_server.load("sprites/meteor_big.png"),
                ..Default::default()
            },
            meteor,
            MovingObject {
                speed: METEOR_LARGE_SPEED,
                direction: 0.0,
                size: (METEOR_LARGE_WIDTH, METEOR_LARGE_HEIGHT),
                hps: METEOR_LARGE_DAMAGE
            }
        )
    );
}

// TODO: add as shared fn among Objects
pub fn meteor_movement(
    mut commands: Commands,
    mut meteor_query: Query<(&mut Transform, Entity, &MovingObject), With<Meteor>>,
    time: Res<Time>
) {
    for (mut transform, entity, object) in meteor_query.iter_mut() {
        // TODO: maybe random y
        let direction = Vec3::new(object.direction, -1.0, 0.0);
        transform.translation += direction * object.speed * time.delta_seconds();
        transform.rotate_z(0.03);

        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    } 
}

pub fn tick_meteor_spawn_timer(
    mut meteor_spawn_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>
) {
    meteor_spawn_timer.timer.tick(time.delta());
}


pub fn spawn_meteors_over_time(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    meteor_spawn_timer: Res<MeteorSpawnTimer>
) {
    if meteor_spawn_timer.timer.finished() {
        spawn_meteor(commands, window_query, asset_server);
    }
}

pub fn spawn_small_meteors(
    commands: &mut Commands,
    location: (f32, f32),
    asset_server: &Res<AssetServer>
) {
    let (x, y) = location;

    for direction in [-0.35, 0.35] {
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/meteor_small.png"),
                    ..Default::default()
                },
                Meteor { 
                    size: MeteorSize::Small
                },
                MovingObject {
                    speed: METEOR_SMALL_SPEED,
                    direction,
                    size: (METEOR_SMALL_WIDTH, METEOR_SMALL_HEIGHT),
                    hps: METEOR_SMALL_DAMAGE
                }
            )
        );
    }

}