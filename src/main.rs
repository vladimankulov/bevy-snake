mod bevy_service;
mod snake;
mod food;

extern crate bevy;

use bevy::app::FixedMain;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::snake::properties::SnakeBody;
use crate::snake::spawn_service::{spawn_snake};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const DEFAULT_BACKGROUND_COLOR: ClearColor = ClearColor(Color::rgb(9.0, 10.0, 11.0));

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("bevy snake"),
                resolution: WindowResolution::new(500., 500.),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(DEFAULT_BACKGROUND_COLOR)
        .add_systems(Startup, spawn_snake)
        .insert_resource(SnakeBody::default())
        .add_systems(Startup, draw_background_field)
        .add_systems(Update, food::spawn_service::spawn_food)
        // .add_systems(Update, button_system)
        .add_systems(Update, snake::move_service::manage_movement)
        .add_systems(Update, snake::property_translation::size_scaling)
        .add_systems(Update, snake::property_translation::position_translation)
        .run();
}

pub fn draw_background_field(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}