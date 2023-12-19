use bevy::ecs::event::Event;

#[derive(Event)]
pub struct HealthChange {}

#[derive(Event)]
pub struct Flash {
    pub display: bool
}