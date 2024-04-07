use bevy::hierarchy::BuildChildren;
use bevy::prelude::{AlignItems, BorderColor, ButtonBundle, Color, Commands, Component, JustifyContent, NodeBundle, Style, TextBundle, Val};
use bevy::prelude::PositionType::Relative;
use bevy::text::TextStyle;
use bevy::ui::{FlexDirection, UiRect};
use bevy::utils::default;
use crate::NORMAL_BUTTON;

#[derive(Component)]
pub struct StartButton {}

#[derive(Component)]
pub struct StatisticsButton {}

#[derive(Component)]
pub struct QuitButton {}

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(construct_button_bundle())
                .with_children(|parent| {
                    parent.spawn(construct_text_bundle(String::from("start")));
                }).insert(QuitButton {});

            parent.spawn(construct_button_bundle()).with_children(|parent| {
                parent.spawn(construct_text_bundle(String::from("statistics")));
            }).insert(StartButton {});

            parent.spawn(construct_button_bundle())
                .with_children(|parent| {
                    parent.spawn(construct_text_bundle(String::from("quit")));
                }).insert(QuitButton {});
        });
}

fn construct_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            top: Val::Px(10.0),
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn construct_text_bundle(value: String) -> TextBundle {
    TextBundle::from_section(value, TextStyle {
        font_size: 20.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    })
}

