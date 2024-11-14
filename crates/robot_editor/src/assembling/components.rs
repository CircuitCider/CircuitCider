use bevy::{ecs::{component::{ComponentHooks, StorageType}, system::SystemState}, prelude::*};
use bevy_mod_outline::OutlineVolume;
use bevy_mod_raycast::prelude::Raycast;
use bevy_rapier3d::rapier::prelude::RigidBody;
use bevy_serialization_extras::prelude::{link::StructureFlag, rigidbodies::RigidBodyFlag};

use crate::{raycast_utils::systems::EXIT_EARLY, ERROR_COLOR, NO_OUTLINE};

// #[derive(Resource, Default)]
// pub struct AssemblingTarget {
//     pub target: Option<Entity>,
//     pub target_structure: Option<StructureFlag>,
// }


const ASSEMBLING_COLOR: Color = Color::LinearRgba(LinearRgba::BLUE);

const ASSEMBLING_OUTLINE: OutlineVolume = OutlineVolume {
    visible: true,
    width: 1.0,
    colour: ASSEMBLING_COLOR
};


#[derive(Default, Debug)]
pub struct AssemblingTarget;

impl Component for AssemblingTarget {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, _| {
            world.commands().add(move |mut world: &mut World| {


                if let Some(mut outline) = world.get_mut::<OutlineVolume>(e) {
                    *outline = ASSEMBLING_OUTLINE
                } else {
                    world.commands().entity(e).insert(ASSEMBLING_OUTLINE);
                }
                
                if let Some(mut rigid_body)  = world.get_mut::<RigidBodyFlag>(e) {
                    *rigid_body = RigidBodyFlag::Fixed
                } 

                let mut state: SystemState<(
                    Query<&mut Transform>,
                    Raycast,
                    Gizmos,
                  )> = SystemState::new(world);
                let (
                    mut transforms, 
                    mut raycast, 
                    mut gizmos
                ) = state.get_mut(&mut world);
                
                let Ok(mut trans) = transforms.get_mut(e) else {return;};
                let ray = raycast.debug_cast_ray(
                    Ray3d::new(trans.translation, Vec3::new(0.0, -1.0, 0.0)), &EXIT_EARLY, &mut gizmos);


                if let Some((_, hit)) = ray.first() {
                    println!("moving robot to {:#?}", hit.barycentric_coord());
                    trans.translation.y = hit.barycentric_coord().y + 1.0;
                }
            });

        });
        _hooks.on_remove(|mut world, e, _| {
            if let Some(mut outline) = world.get_mut::<OutlineVolume>(e) {
                *outline = NO_OUTLINE
            } else {
                world.commands().entity(e).insert(NO_OUTLINE);
            }
            
            if let Some(mut rigid_body)  = world.get_mut::<RigidBodyFlag>(e) {
                *rigid_body = RigidBodyFlag::Dynamic
            } 
        });
    }
}