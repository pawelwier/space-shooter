use bevy::prelude::*;

use crate::{utils::utils::draw_text, game::AppState};

use super::{components::{PlayButton, MainMenu}, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR};

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<State<AppState>>
) {
    let is_game_over = *app_state.get() == AppState::GameOver;
    let (button_text, button_width) = if is_game_over { 
        ("Play again", 500.0) 
    } else { 
        ("Play", 300.0) 
    };

    commands.spawn(
        (
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(50.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu {}
        )
    )
    .with_children(|parent| {
        if is_game_over {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            draw_text(&asset_server, "GAME OVER".to_string(), 100.0),
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..Default::default()
                }
            );
        }
        parent.spawn(
            (
                ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(button_width),
                        height: Val::Px(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                PlayButton {}
            )
        ).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        draw_text(&asset_server, button_text.to_string(), 80.0),
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..Default::default()
            });
        });
    });
}

pub fn react_to_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut bg_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => { app_state_next_state.set(AppState::Game); },
            Interaction::Hovered => { *bg_color = HOVERED_BUTTON_COLOR.into(); },
            Interaction::None => { *bg_color = NORMAL_BUTTON_COLOR.into(); }
        }
    }
}

pub fn despawn_menu(
    mut commands: Commands,
    menu_button_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(menu_entity) = menu_button_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}