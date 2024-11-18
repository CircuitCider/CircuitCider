use bevy::prelude::*;
use super::collision::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, apply_collision_damage);
  }
}
