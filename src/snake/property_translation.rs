use bevy::prelude::{Query, Transform, Vec3, Window, With};
use bevy::window::PrimaryWindow;
use crate::snake::properties::{Position, Size};

pub const ARENA_WIDTH: u32 = 50;
pub const ARENA_HEIGHT: u32 = 50;

pub fn size_scaling(primary_query: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)>) {
    let window = primary_query.get_single().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(sprite_size.width / ARENA_WIDTH as f32 * window.width(),
                                    sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
                                    1.0);
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
    let tile_size = bound_window / grid_count;
    pos / grid_count * bound_window - (bound_window / 2.) + (tile_size / 2.)
}