use bevy::{prelude::*, window::PrimaryWindow};

use crate::object::Object;

use super::{components::{Meteor, MeteorSize}, resources::MeteorSpawnTimer};

pub fn spawn_meteor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(Meteor::get_spawn_x(window.width()), window.height(), 0.0),
                texture: asset_server.load("sprites/meteor_big.png"),
                ..default()
            },
            Meteor { size: MeteorSize::Large }
        )
    );
}

// TODO: add as shared fn among Objects
pub fn meteor_movement(
    mut commands: Commands,
    mut meteor_query: Query<(&mut Transform, Entity, &mut Meteor), With<Meteor>>,
    time: Res<Time>
) {
    for (mut transform, entity, meteor) in meteor_query.iter_mut() {
        // TODO: maybe random y
        let direction = Vec3::new(0.0, -1.0, 0.0);
        transform.translation += direction * Meteor::speed(&meteor) * time.delta_seconds();
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