use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Health, Bullet, CollisionDamage, Velocity};

pub fn apply_collision_damage(
  collision_damage_query: Query<(Entity, &CollisionDamage)>,
  mut health_query: Query<&mut Health>,
  //rapier_context: Res<RapierContext>,
  name_query: Query<&Bullet>,
  mut commands: Commands,
  ) {
//   for (e, damage) in collision_damage_query.iter() {
//         for (collided, collider, _) in rapier_context.intersection_pairs_with(e)
//         //.filter(|(_, _, bool)| bool == &true)
//         {
//             let e_target = collided;
//             let Ok(mut health) = health_query.get_mut(e_target) else {
//                 return;
//             };
            
//             health.hp -= damage.0;
//             if name_query.get(collider).is_ok() {
//                 commands.entity(collider).despawn_recursive();
//             }
//         }
//     }
}
pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_secs();
    }
}
