use bevy::prelude::{Query, Transform, Vec3, Window, With};
use bevy::window::PrimaryWindow;
use crate::snake::properties::{Position, Size};

const ARENA_WIDTH: u32 = 100;
const ARENA_HEIGHT: u32 = 100;

pub fn size_scaling(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)>) {
    let window = primary_query.get_single().unwrap();
    let cell_width = window.width() / ARENA_WIDTH as f32;
    let cell_height = window.height() / ARENA_HEIGHT as f32;

    for (sprite_size, mut transform) in q.iter_mut() {
        // Calculate scale factors
        let scale_x = cell_width * sprite_size.width;
        let scale_y = cell_height * sprite_size.height;

        // Apply scale
        transform.scale = Vec3::new(scale_x, scale_y, 0.0);
    }
}

pub fn position_translation(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    let window = primary_query.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn convert(pos: f32, bound_window: f32, grid_count: f32) -> f32 {
    let cell_size = bound_window / grid_count;
    pos * cell_size - (bound_window / 2.0) + (cell_size / 2.0)
}