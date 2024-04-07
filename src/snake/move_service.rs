use bevy::input::ButtonInput;

use bevy::prelude::{Entity, KeyCode, Query, Res, ResMut};
use bevy::prelude::KeyCode::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
use crate::snake::properties::{Head, Position, SnakeBody};
use crate::snake::properties::Direction::{DOWN, LEFT, RIGHT, UP};

pub fn manage_movement(mut body: ResMut<SnakeBody>,
                       keyboard_input: Res<ButtonInput<KeyCode>>,
                       mut query: Query<(Entity, &mut Head)>,
                       mut positions: Query<&mut Position>) {
    if let Some((head_entity, mut head)) = query.iter_mut().next() {
        if !keyboard_input.any_pressed([ArrowUp, ArrowRight, ArrowDown, ArrowLeft]) {
            move_snake(body, positions, head_entity, head.as_ref());
            return;
        }
        if keyboard_input.pressed(ArrowUp) {
            head.direction = Some(UP);
        } else if keyboard_input.pressed(ArrowDown) {
            head.direction = Some(DOWN);
        } else if keyboard_input.pressed(ArrowLeft) {
            head.direction = Some(LEFT);
        } else if keyboard_input.pressed(ArrowRight) {
            head.direction = Some(RIGHT);
        }
        move_snake(body, positions, head_entity, head.as_ref());
    }
}

fn move_snake(mut body: ResMut<SnakeBody>, mut positions: Query<&mut Position>, head_entity: Entity, head: &Head) {
    let mut head_position = positions.get_mut(head_entity).unwrap();
    let mut old_pos = Position::from(head_position.x, head_position.y);

    match &head.direction {
        Some(direction) => {
            match direction {
                UP => { head_position.y += 1; }
                DOWN => { head_position.y -= 1; }
                RIGHT => { head_position.x += 1; }
                LEFT => { head_position.x -= 1; }
            }
        }
        None => {}
    }

    for e in body.iter() {
        if e.index() == head_entity.index() {
            continue;
        }
        let mut position = positions.get_mut(*e).unwrap();
        let holder = *position;
        position.x = old_pos.x;
        position.y = old_pos.y;
        old_pos = holder;
    }
}