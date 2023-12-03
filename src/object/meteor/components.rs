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
    pub size: MeteorSize
}

impl Object for Meteor {
    fn speed(&self) -> f32 {
        return if self.size == MeteorSize::Large { METEOR_LARGE_SPEED } else { METEOR_SMALL_SPEED };
    }
}