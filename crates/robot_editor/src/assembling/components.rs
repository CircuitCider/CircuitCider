use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        system::SystemState,
    },
    prelude::*,
};
use bevy_mod_outline::OutlineVolume;
use bevy_serialization_extras::prelude::rigidbodies::RigidBodyFlag;

use crate::{
    raycast_utils::systems::EXIT_EARLY, NO_OUTLINE,
};

// #[derive(Resource, Default)]
// pub struct AssemblingTarget {
//     pub target: Option<Entity>,
//     pub target_structure: Option<StructureFlag>,
// }

const ASSEMBLING_COLOR: Color = Color::LinearRgba(LinearRgba::BLUE);

const ASSEMBLING_OUTLINE: OutlineVolume = OutlineVolume {
    visible: true,
    width: 1.0,
    colour: ASSEMBLING_COLOR,
};

#[derive(Default, Debug, Reflect)]
pub struct AssemblingTarget;

impl Component for AssemblingTarget {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, _| {
            world.commands().queue(move |mut world: &mut World| {
                if let Some(mut outline) = world.get_mut::<OutlineVolume>(e) {
                    *outline = ASSEMBLING_OUTLINE
                } else {
                    world.commands().entity(e).insert(ASSEMBLING_OUTLINE);
                }

                if let Some(mut rigid_body) = world.get_mut::<RigidBodyFlag>(e) {
                    *rigid_body = RigidBodyFlag::Fixed
                }

                let mut state: SystemState<(Query<&mut Transform>, MeshRayCast, Gizmos)> =
                    SystemState::new(world);
                let (mut transforms, mut raycast, _) = state.get_mut(&mut world);

                let Ok(mut trans) = transforms.get_mut(e) else {
                    return;
                };
                //TODO: Re-make this a debug raycast if those get added back
                let ray = raycast.cast_ray(
                    Ray3d::new(
                        trans.translation,
                        Dir3::new_unchecked(Vec3::new(0.0, -1.0, 0.0)),
                    ),
                    &EXIT_EARLY,
                );

                if let Some((_, hit)) = ray.first() {
                    println!("moving robot to {:#?}", hit.barycentric_coords);
                    trans.translation.y = hit.barycentric_coords.y + 1.0;
                }
            });
        });
        _hooks.on_remove(|mut world, e, _| {
            if let Some(mut outline) = world.get_mut::<OutlineVolume>(e) {
                *outline = NO_OUTLINE
            } else {
                world.commands().entity(e).insert(NO_OUTLINE);
            }

            if let Some(mut rigid_body) = world.get_mut::<RigidBodyFlag>(e) {
                *rigid_body = RigidBodyFlag::Dynamic
            }
        });
    }
}
