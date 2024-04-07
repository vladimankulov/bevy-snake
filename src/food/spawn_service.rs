use bevy::prelude::{Color, Commands, EventReader, Query, Sprite};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy::utils::petgraph::visit::Walker;
use rand::random;
use crate::food::properties::{Food, SpawnNewFood};
use crate::snake::properties::{Position, Size};
use crate::snake::property_translation::{ARENA_HEIGHT, ARENA_WIDTH};

pub fn spawn(mut commands: Commands) {
    commands.spawn(food_bundle())
        .insert(Position::from((random::<f32>() * ARENA_WIDTH as f32) as i32, (random::<f32>() * ARENA_HEIGHT as f32) as i32))
        .insert(Size::square(0.8))
        .insert(Food {});
}

pub fn spawn_food(commands: Commands, mut event_reader: EventReader<SpawnNewFood>) {
    if !event_reader.is_empty() {
        spawn(commands);
        event_reader.clear();
    }
}

pub fn food_bundle() -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            ..default()
        },
        ..default()
    }
}