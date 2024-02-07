use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(direction: Vec2) -> Self {
        Self(direction)
    }
}
