use bevy::ecs::component::Component;
use rand::random;

pub mod meteor;
pub mod enemy;

pub trait Object : Sized {
    fn speed(&self) -> f32;
    fn get_direction(&self) -> f32;

    fn get_spawn_x(&self, window_width: f32) -> f32 {
        random::<f32>() * window_width
    }
}

#[derive(Component)]
pub struct MovingObject {}