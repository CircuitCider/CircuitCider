use bevy::{ecs::component::{ComponentHooks, StorageType}, prelude::*};
use bevy_mod_outline::OutlineVolume;
use bevy_serialization_extras::prelude::rigidbodies::RigidBodyFlag;
use shader_core::shaders::neon::NeonMaterial;

use crate::NO_OUTLINE;


const ATTACHING_COLOR: Color = Color::LinearRgba(LinearRgba::GREEN);

const ATTACHING_OUTLINE: OutlineVolume = OutlineVolume {
    visible: true,
    width: 1.0,
    colour: ATTACHING_COLOR
};

/// marker for objects that are not yet a part of a structure but could be
/// (placed build mode models)
#[derive(Default)]
pub struct AttachCandidate {
    pub attempt_target: Option<Entity>
}

impl Component for AttachCandidate {
    const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, _| {
            world.commands().add(move |mut world: &mut World| {


                if let Some(mut outline) = world.get_mut::<OutlineVolume>(e) {
                    *outline = ATTACHING_OUTLINE
                } else {
                    world.commands().entity(e).insert(ATTACHING_OUTLINE);
                }
                // don't re-add neon material if its alrady there. its color is managed by intersection checks.
                if world.get_mut::<Handle<NeonMaterial>>(e).is_none() {
                    if let Some(handle) = world.get_resource_mut::<Assets<NeonMaterial>>()
                    .map(|mut neon_mats| neon_mats.add(NeonMaterial::default())) {
                        world.commands().entity(e).insert(handle);

                    }
                }


                if let Some(mut rigid_body)  = world.get_mut::<RigidBodyFlag>(e) {
                    *rigid_body = RigidBodyFlag::Fixed
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


// pub struct Attach