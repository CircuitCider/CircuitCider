use super::fire_shot::*;
use crate::{
    components::SpawnTimer,
    weapons::plugins::CollisionPlugin,
};
use bevy::prelude::*;

pub struct BulletPlugin;
pub const SPAWN_TIME_SECONDS: f32 = 0.5;
//Update constant to change bullet fire rate
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, fire_bullet);
    }
}

/// plugin for all combat functionality
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BulletPlugin).add_plugins(CollisionPlugin);
    }
}
