use bevy::prelude::*;

/// Health points.
#[derive(Component)]
pub struct Health {
    pub hp: f32,
}
impl Health {
    pub fn new(hp: f32) -> Self {
        Self { hp }
    }
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

#[derive(Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component)]
pub struct Pistol;