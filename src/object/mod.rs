use bevy::ecs::component::Component;
use rand::random;

pub mod meteor;
pub mod enemy;
pub mod powerup;

pub trait SpawnedEntity : Sized {
    fn get_spawn_x(&self, window_width: f32) -> f32 {
        random::<f32>() * window_width
    }
}

#[derive(Component)]
pub struct MovingObject {
    pub speed: f32,
    pub direction: f32,
    pub size: (f32, f32),
    pub damage: f32
}