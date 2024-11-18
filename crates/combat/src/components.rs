use bevy::prelude::*;

/// Health points.
#[derive(Component)]
pub struct Health {
    pub hp: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            hp: 100.0,
        }
    }
}

#[derive(Component)]
pub struct CollisionDamage(pub f32);

#[derive(Component)]
pub struct Bullet;
