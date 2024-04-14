use bevy::prelude::*;

pub mod resources;
pub mod systems;
pub mod components;
pub mod events;

pub const WINDOW_WIDTH: f32 = 900.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}