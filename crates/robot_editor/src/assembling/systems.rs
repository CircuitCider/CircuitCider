use bevy::{prelude::*, transform::commands};
use bevy_mod_outline::OutlineVolume;
use bevy_serialization_extras::prelude::rigidbodies::RigidBodyFlag;

use crate::raycast_utils::systems::EXIT_EARLY;

use super::components::AssemblingTarget;

// /// managed transition to assembling mode
// pub fn transition_to_assembler(
//     mut outlines: Query<&mut OutlineVolume>,
//     mut targets: Query<(Entity, &mut Transform, Option<&mut RigidBodyFlag>), Added<AssemblingTarget>>,
//     mut commands: Commands,
//     mut raycast: Raycast,
//     mut gizmos: Gizmos,
// ) {
//     // let Some(target) = target.target else {return;};

//     let color = Color::LinearRgba(LinearRgba::BLUE);
//     let new_outline = OutlineVolume {
//         visible: true,
//         width: 1.0,
//         colour: color
//     };
//     for (e, mut trans, mut rigid_body) in targets.iter_mut() {
//         if let Ok(mut outline) = outlines.get_mut(e) {
//             *outline = new_outline.clone();
//         } else {
//             commands.entity(e).insert(new_outline.clone());
//         }
//         if let Some(mut rigid_body)  = rigid_body{
//             *rigid_body = RigidBodyFlag::Fixed
//         }

//         let ray = raycast.debug_cast_ray(
//             Ray3d::new(trans.translation, Vec3::new(0.0, -1.0, 0.0)), &EXIT_EARLY, &mut gizmos);

//         if let Some((_, hit)) = ray.first() {
//             println!("moving robot to {:#?}", hit.barycentric_coord());
//             trans.translation.y = hit.barycentric_coord().y + 1.0;
//         }
//     }

// }
