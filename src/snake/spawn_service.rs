use bevy::prelude::{Color, Commands, default, ResMut, Sprite, SpriteBundle, Vec2};
use crate::snake::properties::{Body, Head, Position, Size, SnakeBody};

const BLOCK: Vec2 = Vec2 { x: 20., y: 20. };

pub fn spawn_snake(mut commands: Commands, mut snake: ResMut<SnakeBody>) {
    let snake_head = commands.spawn(spawn_head_block())
        .insert(Head::default())
        .insert(Body {})
        .insert(Position::from_x(3))
        .insert(Size::square(2.))
        .id();

    let snake_body = commands.spawn(spawn_body_block())
        .insert(Body {})
        .insert(Position::from(3, 2))
        .insert(Size::square(2.))
        .id();

    snake.add(snake_head);
    snake.add(snake_body);
}

fn spawn_head_block() -> SpriteBundle {
    spawn_snake_block(Color::BLUE)
}

fn spawn_body_block() -> SpriteBundle {
    spawn_snake_block(Color::BLACK)
}

fn spawn_snake_block(color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            // custom_size: Some(BLOCK),
            ..default()
        },
        ..default()
    }
}