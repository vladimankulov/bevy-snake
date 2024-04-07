use std::collections::linked_list::{Iter, IterMut};
use std::collections::LinkedList;
use bevy::prelude::{Component, Entity, Resource};

#[derive(Component)]
pub struct Head {
    pub direction: Option<Direction>,
}

#[derive(Component)]
pub struct Body {}

#[derive(Component)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct SnakeBody(LinkedList<Entity>);

impl Default for SnakeBody {
    fn default() -> Self {
        SnakeBody(LinkedList::new())
    }
}

impl SnakeBody {
    pub fn add(&mut self, entity: Entity) {
        self.0.push_back(entity);
    }
    pub fn iter(&self) -> Iter<'_, Entity> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Entity> {
        self.0.iter_mut()
    }
}

impl Size {
    pub fn square(value: f32) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}

impl Position {
    pub fn from_x(x: i32) -> Self {
        Position { x, y: x }
    }

    pub fn from(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Default for Head {
    fn default() -> Self {
        Head { direction: None }
    }
}