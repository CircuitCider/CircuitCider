use bevy::prelude::*;
use super::fire_shot::*;
use crate::components::SpawnTimer;

pub struct BulletPlugin;
pub const SPAWN_TIME_SECONDS: f32 = 0.1;
//Update constant to change bullet fire rate
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode:: Repeating)
        })
            .add_systems(Update, fire_bullet);
    }
}
