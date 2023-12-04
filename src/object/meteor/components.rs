use bevy::prelude::Component;

use crate::object::Object;

use super::{METEOR_LARGE_SPEED, METEOR_SMALL_SPEED};

#[derive(PartialEq)]
pub enum MeteorSize {
    Large,
    Small
}

#[derive(Component)]
pub struct Meteor {
    pub size: MeteorSize,
    pub direction: f32
}

impl Object for Meteor {
    fn speed(&self) -> f32 {
        if self.size == MeteorSize::Large { METEOR_LARGE_SPEED } else { METEOR_SMALL_SPEED }
    }

    fn get_direction(&self) -> f32 {
        self.direction
    }
}