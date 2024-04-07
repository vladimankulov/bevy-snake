use bevy::prelude::{Commands, Entity, EventWriter, Query, With};
use crate::food::properties::{Food, SpawnNewFood};
use crate::snake::properties::{GrowthEvent, Head, Position};

pub fn eat_food_management(
    mut commands: Commands,
    mut spawn_food_event: EventWriter<SpawnNewFood>,
    mut growth_snake_event: EventWriter<GrowthEvent>,
    food_position_query: Query<(Entity, &Position), With<Food>>,
    head_position_query: Query<&Position, With<Head>>) {
    for (food_entity, food_position) in food_position_query.iter() {
        for (head_position) in head_position_query.iter() {
            if food_position == head_position {
                commands.entity(food_entity).despawn();
                spawn_food_event.send(SpawnNewFood {});
                growth_snake_event.send(GrowthEvent {});
            }
        }
    }
}