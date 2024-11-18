use bevy::{
    ecs::component::Component,
};

/// Health points.
#[derive(Component)]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            hp: 100,
            max_hp: 100,
        }
    }
}

#[derive(Component)]
pub struct CollisionDamage(pub f32);

#[derive(Component)]
pub struct Bullet;
