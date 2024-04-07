use bevy::prelude::{Color, Commands, Query, Sprite};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use crate::food::properties::Food;
use crate::snake::properties::{Position, Size};

pub fn spawn_food(mut commands: Commands, mut query: Query<&Food>) {
    let food_option = query.iter().next();
    if food_option.is_some() {
        return;
    }
    commands.spawn(food_bundle())
        .insert(Position::from(4, 4))
        .insert(Size::square(2.))
        .insert(Food {});
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