use bevy::prelude::*;
use super::systems::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, apply_collision_damage)
    .add_systems(Update, (update_position, update_velocity));
  }
}
