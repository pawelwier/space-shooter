use bevy::prelude::*;

use super::{components::ScoreComponent, resources::ScoreResource};

fn draw_score_text(
    asset_server: &AssetServer,
    text: String
) -> TextSection {
    TextSection::new(
        text,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 70.0,
            ..default()
        },
    )
}

pub fn add_points_to_score(
    score: f32,
    score_resource: &mut ResMut<ScoreResource>,
    score_query: &mut Query<&mut Text, With<ScoreComponent>>,
) {
    score_resource.points += score;

    for mut text in score_query.iter_mut() {
        text.sections[1].value = score_resource.points.to_string();
    } 
}

pub fn spawn_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..Default::default()
        },
        ..Default::default()
    }).with_children(
        |parent| {
            parent.spawn((
                TextBundle::from_sections([
                    draw_score_text(
                        &asset_server,
                        "SCORE: ".to_string()
                    ),
                    draw_score_text(
                        &asset_server,
                        "0".to_string()
                    ),
                ]),
                ScoreComponent {},
            ));
        }
    );
}