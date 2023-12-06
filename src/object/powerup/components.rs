use bevy::prelude::Component;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::object::SpawnedEntity;

#[derive(PartialEq, Debug, Clone)]
pub enum PowerUpType {
    Bolt,
    Shield,
    Star
}

#[derive(Component, PartialEq, Debug)]
pub struct PowerUp {
    pub power_type: PowerUpType
}

impl SpawnedEntity for PowerUp {}

pub fn get_power_up_details(power_up_type: &PowerUpType) -> &'static str {
    match power_up_type {
        PowerUpType::Bolt => {
            println!("it's a bolt");
            "sprites/power_up_bolt.png"
        }
        PowerUpType::Shield => {
            println!("it's a shield");
            "sprites/power_up_shield.png"
        }
        PowerUpType::Star => {
            println!("it's a star");
            "sprites/power_up_star.png"
        }
    }
}

impl Distribution<PowerUpType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUpType {
        match rng.gen_range(0..=2) {
            0 => PowerUpType::Bolt,
            1 => PowerUpType::Shield,
            _ => PowerUpType::Star,
        }
    }
}

pub fn get_random_power_type() -> PowerUpType {
    let power_type = rand::random();
    power_type
}