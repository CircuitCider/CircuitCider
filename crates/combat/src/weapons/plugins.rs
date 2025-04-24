use super::systems::*;
use bevy::prelude::*;
use bevy::animation::animate_targets;

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_collision_damage)
            .add_systems(Update, update_position);
    }
}

pub struct MacePlugin;
impl Plugin for MacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, light_attack)
        .add_systems(Update, mace_light_animation.before(animate_targets))
        .add_systems(Update, mouse_animation_control);
    }
}