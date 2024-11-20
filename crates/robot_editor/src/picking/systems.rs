use bevy::{input::keyboard::KeyboardInput, math::bounding::{IntersectsVolume, RayCast3d}, pbr::wireframe::Wireframe, prelude::*};
use bevy_mod_outline::{OutlineBundle, OutlineVolume};
use bevy_mod_picking::{
    focus::PickingInteraction, highlight::PickHighlight, picking_core::{Pickable, PickingPluginsSettings},
    selection::PickSelection, PickableBundle,
};
use bevy_mod_raycast::prelude::{Raycast, RaycastSettings};
use bevy_serialization_extras::prelude::{link::{JointFlag, StructureFlag}, rigidbodies::RigidBodyFlag};
use shader_core::shaders::flow_wireframe::FlowWireframeMaterial;
use transform_gizmo_bevy::GizmoTarget;

use crate::{assembling::components::AssemblingTarget, attaching::components::AttachCandidate, placing::components::Placer, raycast_utils::{resources::CursorRayHits, systems::{DONT_EXIT_EARLY, EXIT_EARLY}}, resources::BuildToolMode};


pub fn toggle_picking_enabled(
    gizmo_targets: Query<&GizmoTarget>,
    mut picking_settings: ResMut<PickingPluginsSettings>,
) {
    // Picking is disabled when any of the gizmos is focused or active.

    picking_settings.is_enabled = gizmo_targets
        .iter()
        .all(|target| !target.is_focused() && !target.is_active());
}


/// effects on things that are iteracted with
pub fn picking_interaction_effects(
    interactables: Query<(
        Entity,
        Option<&StructureFlag>,
        &PickingInteraction
    ), Changed<PickingInteraction>>,
    //hovered: Query<&Hovered>,
    mut commands: Commands,
    hits: ResMut<CursorRayHits>,
    mouse: Res<ButtonInput<MouseButton>>,
) {

    let Some((_, _, (e, structure, interaction))) = hits.first_with(&interactables)  else {return;};
    
    
    if interaction == &PickingInteraction::Pressed && mouse.just_pressed(MouseButton::Left) {
        let structure_exists = structure.map(|_| true).unwrap_or(false);

        if structure_exists  == false{
            //TODO: This is not correct, this will only work for hulls. 
            commands.entity(e).insert(Placer::Hull);
        }
    }
    // if interaction == &PickingInteraction::Hovered {
        
    // }
}

/// effects on things that are clicked on
pub fn picking_click_effects(
    mut commands: Commands,
    mut keys: Res<ButtonInput<KeyCode>>,
    mut clickables: Query<(
        Entity,
        Option<&StructureFlag>,
        &PickSelection,
    ), Changed<PickSelection>>,
    mut structures: Query<(Entity, &StructureFlag)>,
    joints: Query<&JointFlag>,
    // mut rigid_bodies: Query<&mut RigidBodyFlag>,
    // mut outlines: Query<&mut OutlineVolume>,
    // mut tool_mode: ResMut<NextState<BuildToolMode>>,
    // mut assembling_target_structure: Query<(&StructureFlag, &AssemblingTarget)>

) {
    // Continuously update entities based on their picking state

    for (e, structure, selectable) in &mut clickables {

        if selectable.is_selected {
            let mut entity_cmd = commands.entity(e);

            // entity_cmd.insert(GizmoTarget::default());

            if keys.pressed(KeyCode::ShiftLeft) {
                entity_cmd.insert(AssemblingTarget);
            } 
            else if let Ok(joint) = joints.get(e) {
                //if let Ok(joint) = joints.get(e) {
                entity_cmd.insert(AttachCandidate {
                    attempt_target: joint.parent_id
                });
            } else {
                entity_cmd.insert(GizmoTarget::default());
            }

                //}
                // if let Some(structure) = structure {
                    
                //     //if structures.iter().find(|(e_target, n)| n.name == structure.name)
                //     // if structures.iter().any(||n.name == structure.name) {
                //     //     entity_cmd.insert(AttachCandidate {
                //     //         attempt_target: Some(e)
                //     //     });
                //     // }
                // }
        } else {
            let mut entity_cmd = commands.entity(e);

            entity_cmd.remove::<GizmoTarget>();
            // entity_cmd.remove::<AssemblingTarget>();
            // let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Dynamic);
            // if let Ok(mut outline) = outlines.get_mut(e) {
            //     outline.visible = false;
                
            // }            
        }
    }

    //TODO: re-attempt to implement this to auto-freeze attached components
    // for (entity, structure) in &mut targets {
    //     let Ok(selectable) = selectables.get(entity) else {return;};
    //     //let Some((hit, ..)) = hits.0.first() else {return;};
    //     if selectable.is_selected {
    //         let mut total_targets = Vec::new();
        
    //         // add all connected structures to list
    //         if let Some(structure) = structure {
    //             structures.iter()
    //             .filter(|(_, candidate)| candidate.name == structure.name)
    //             .for_each(|target| total_targets.push(target.0));
    //         } 
    //         total_targets.push(entity);
            
    //         println!("total targets: {:#?}", total_targets);
    //         for e in total_targets {
    //             let mut entity_cmd = commands.entity(e);

    //             entity_cmd.insert(GizmoTarget::default());
                
    //             let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Fixed);
    //             let _ = outlines.get_mut(e).map(|mut outline| outline.visible = true);
    //             if let Ok(mut selectable) = selectables.get_mut(e) {
    //                 selectable.is_selected = true;
    //             }
    //         }
            

    //     } else if selectable.is_selected == false {
    //         println!("unpicking");
    //         let mut total_targets = Vec::new();
        
    //         // add all connected structures to list
    //         if let Some(structure) = structure {
    //             structures.iter()
    //             .filter(|(_, candidate)| candidate.name == structure.name)
    //             .for_each(|target| total_targets.push(target.0));
    //         } 
    //         total_targets.push(entity);
            
    //         for e in total_targets {
    //             let mut entity_cmd = commands.entity(e);

    //             entity_cmd.remove::<GizmoTarget>();
                
    //             let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Fixed);
    //             let _ = outlines.get_mut(e).map(|mut outline| outline.visible = false);
    //         }
            
    //     }
    // }
}


pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (With<StructureFlag>, Without<Pickable>)>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert((
            PickableBundle {
                pickable: Pickable::default(),
                interaction: PickingInteraction::default(),
                selection: PickSelection::default(),
                highlight: PickHighlight::default(),
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: false,
                    colour: Color::Srgba(Srgba::GREEN),
                    width: 2.0,
                },
                ..default()
            },
        ));
    }
}
