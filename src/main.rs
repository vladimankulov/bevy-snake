mod bevy_service;
mod snake;
mod food;

extern crate bevy;

use std::time::Duration;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::utils::tracing::instrument::WithSubscriber;
use bevy::window::WindowResolution;
use crate::food::properties::SpawnNewFood;
use crate::food::spawn_service::{spawn, spawn_food};
use crate::snake::eat_service::eat_food_management;
use crate::snake::move_service::manage_movement;
use crate::snake::properties::{GrowthEvent, SnakeBody, Tail};
use crate::snake::property_translation::{position_translation, size_scaling};
use crate::snake::spawn_service::{growth_management, spawn_snake};

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
        .add_systems(Startup, (draw_background_field, spawn_snake, spawn).chain())
        .add_systems(PostUpdate, (size_scaling, position_translation).chain())
        .insert_resource(SnakeBody::default())
        .insert_resource(Tail::default())
        .add_systems(FixedUpdate, (manage_movement, eat_food_management.after(manage_movement), growth_management.after(eat_food_management), spawn_food.after(eat_food_management)))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(150)))
        .add_event::<GrowthEvent>()
        .add_event::<SpawnNewFood>()
        // .add_systems(PreUpdate, spawn_food)
        .run();
}

pub fn draw_background_field(mut command: Commands) {
    command.spawn(Camera2dBundle::default());
}