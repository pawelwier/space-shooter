use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct Explosion {
    pub timer: Timer
}