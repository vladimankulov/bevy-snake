use bevy::input::ButtonInput;

use bevy::prelude::{Commands, Entity, EventReader, KeyCode, Query, Res, ResMut};
use bevy::prelude::KeyCode::{ArrowDown, ArrowLeft, ArrowRight, ArrowUp};
use crate::snake::properties::{GrowthEvent, Head, Position, SnakeBody, Tail};
use crate::snake::properties::Direction::{DOWN, LEFT, RIGHT, UP};

pub fn manage_movement(body: ResMut<SnakeBody>,
                       tail: ResMut<Tail>,
                       keyboard_input: Res<ButtonInput<KeyCode>>,
                       mut query: Query<(Entity, &mut Head)>,
                       positions: Query<&mut Position>) {
    if let Some((head_entity, mut head)) = query.iter_mut().next() {
        if keyboard_input.pressed(ArrowUp) {
            head.set_direction_if_not_opposite(UP);
        } else if keyboard_input.pressed(ArrowDown) {
            head.set_direction_if_not_opposite(DOWN);
        } else if keyboard_input.pressed(ArrowLeft) {
            head.set_direction_if_not_opposite(LEFT);
        } else if keyboard_input.pressed(ArrowRight) {
            head.set_direction_if_not_opposite(RIGHT);
        }
        move_snake(body, tail, positions, head_entity, head.as_ref());
    }
}



fn move_snake(mut body: ResMut<SnakeBody>,
              mut tail: ResMut<Tail>,
              mut positions: Query<&mut Position>, head_entity: Entity, head: &Head) {
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
    tail.0 = Some(old_pos);
}