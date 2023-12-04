use bevy::prelude::Component;

pub enum PlayerMovement {
    None,
    Left,
    Right
}

#[derive(Component)]
pub struct Player {
    pub movement: PlayerMovement
}

#[derive(Component)]
pub struct Laser {}