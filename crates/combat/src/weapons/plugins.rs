use super::systems::*;
use bevy::prelude::*;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_collision_damage)
            .add_systems(Update, update_position);
    }
}
