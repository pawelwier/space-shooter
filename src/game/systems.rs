use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::resources::PlayerParams;

use super::{
    resources::ScoreResource, 
    events::{HealthChange, Flash},
    components::{ScoreComponent, FlashComponent}
};

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

pub fn spawn_health_bar_init(
    mut commands: Commands
) {
    spawn_health_bar(&mut commands, 100.0);
}

pub fn update_health(
    mut commands: Commands,
    mut health_change_event_reader: EventReader<HealthChange>,
    health_resource: ResMut<PlayerParams>
) {
    for _ in health_change_event_reader.read() {
        spawn_health_bar(
            &mut commands,
            health_resource.health,
        )
    }
}

pub fn spawn_health_bar(
    commands: &mut Commands,
    health: f32,
) {
    let health_bar_width_px = 300.0;
    let hp_width_px = health_bar_width_px / 100.0;
    let health_left_px = hp_width_px * health;

    commands.spawn(
        NodeBundle {
            style: Style {
                left: Val::Px(490.0),
                top: Val::Px(10.0),
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                ..Default::default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        }
    ).clear_children()
    .with_children(
        |parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        left: Val::Px(0.0),
                        top: Val::Px(0.0),
                        width: Val::Px(health_left_px),
                        height: Val::Px(50.0),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.2, 0.5, 0.5).into(),
                    ..Default::default()
                }
            );
        }
    );
}

pub fn spawn_flash_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut flash_event_reader: EventReader<Flash>,
    flash_query: Query<Entity, With<FlashComponent>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for event in flash_event_reader.read() {
        if event.display {
            commands.spawn(
                (
                    SpriteBundle {
                        transform: Transform { 
                            translation: Vec3::new(
                                window.width() - 30.0,
                                window.height() - 100.0,
                                0.0
                            ), 
                            scale: Vec3::new(1.5, 1.5, 0.0),
                            ..Default::default()
                        },
                        texture: asset_server.load("sprites/bolt_gold.png"),
                        ..Default::default()
                    },
                    FlashComponent {}
                )
            );
        } else {
            for entity in flash_query.iter() {                
                commands.entity(entity).despawn();
            }
        }
    }
}