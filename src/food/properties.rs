use bevy::prelude::{Component, Event};

#[derive(Component)]
pub struct Food {}

#[derive(Event)]
pub struct SpawnNewFood {}