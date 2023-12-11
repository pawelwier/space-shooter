use std::time::Duration;

use bevy::prelude::*;

use super::components::Explosion;

pub fn spawn_explosion(
    commands: &mut Commands,
    location: (f32, f32),
    asset_server: &Res<AssetServer>
) {
    let (x, y) = location;
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/explosion.png"),
                ..Default::default()
            },
            Explosion {
                timer: Timer::new(Duration::new(0, 400000000), TimerMode::Once)
            }
        )
    );
}

pub fn despawn_explosions_on_timeout(
    mut commands: Commands,
    mut explosion_query: Query<(&mut Explosion, Entity), With<Explosion>>,
    time: Res<Time>,
) {
    for (mut explosion, explosion_entity) in explosion_query.iter_mut() {
        explosion.timer.tick(time.delta());
        if explosion.timer.finished() {
            commands.entity(explosion_entity).despawn();
        }
    }
}