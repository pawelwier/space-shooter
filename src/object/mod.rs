use rand::random;

pub mod meteor;

pub trait Object {
    fn speed(&self) -> f32;

    fn get_spawn_x(window_width: f32) -> f32 {
        random::<f32>() * window_width
    }
}
