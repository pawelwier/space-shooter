use bevy::prelude::Component;

use crate::object::SpawnedEntity;

#[derive(PartialEq)]
pub enum MeteorSize {
    Large,
    Small
}

#[derive(Component, PartialEq)]
pub struct Meteor {
    pub size: MeteorSize,
}

impl SpawnedEntity for Meteor {}