use bevy::prelude::Component;

use crate::object::Object;

use super::ENEMY_SHIP_SPEED;

#[derive(Component, PartialEq)]
pub struct Enemy {
    pub direction: f32
}

impl Object for Enemy {
    fn speed(&self) -> f32 {
        ENEMY_SHIP_SPEED
    }

    fn get_direction(&self) -> f32 {
        self.direction
    }
}