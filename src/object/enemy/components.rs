use bevy::prelude::Component;

use crate::object::SpawnedEntity;

#[derive(Component, PartialEq)]
pub struct Enemy {}

impl SpawnedEntity for Enemy {}

#[derive(Component)]
pub struct EnemyLaser {}

impl SpawnedEntity for EnemyLaser {}