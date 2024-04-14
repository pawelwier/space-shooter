use bevy::prelude::*;

pub fn draw_text(
    asset_server: &AssetServer,
    text: String,
    font_size: f32
) -> TextSection {
    TextSection::new(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            color: Color::SILVER,
            font_size,
            ..default()
        },
    )
}