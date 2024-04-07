use bevy::prelude::{Color, Commands, default, EventReader, ResMut, Sprite, SpriteBundle, Vec2};
use crate::snake::properties::{Body, GrowthEvent, Head, Position, Size, SnakeBody, Tail};

pub fn spawn_snake(mut commands: Commands, mut snake: ResMut<SnakeBody>) {
    let snake_head = commands.spawn(spawn_head_block())
        .insert(Head::default())
        .insert(Body {})
        .insert(Position::from_x(3))
        .insert(Size::square(0.8))
        .id();

    let snake_body = commands.spawn(spawn_body_block())
        .insert(Body {})
        .insert(Position::from(3, 4))
        .insert(Size::square(0.8))
        .id();
    snake.add(snake_head);
    snake.add(snake_body);
}

pub fn growth_management(mut commands: Commands,
                         mut event_reader: EventReader<GrowthEvent>,
                         mut body: ResMut<SnakeBody>,
                         mut tail: ResMut<Tail>) {
    if event_reader.is_empty() {
        return;
    }
    body.add(commands.spawn(spawn_body_block())
        .insert(Body {})
        .insert(tail.0.unwrap())
        .insert(Size::square(0.8))
        .id());
    event_reader.clear();
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
            ..default()
        },
        ..default()
    }
}