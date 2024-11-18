use bevy::prelude::*;
use super::fire_shot::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_bullet);
    }
}