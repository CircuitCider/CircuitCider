use bevy::{
    picking::focus::PickingInteraction,
    prelude::*,
};
use bevy_mod_outline::OutlineVolume;
use bevy_picking::pointer::{PointerInteraction, PointerPress};
use bevy_serialization_assemble::{Assemblies, AssemblyId};
use bevy_serialization_extras::prelude::JointFlag;
use transform_gizmo_bevy::GizmoTarget;

use crate::{
    assembling::components::AssemblingTarget, attaching::components::AttachCandidate, placing::components::Placer, systems::first_valid_other_hit, Part
    // raycast_utils::resources::CursorRayHits,
};

use super::{components::{PickCollector, PickSelected}, PickMode};

pub fn toggle_picking_enabled(
    gizmo_targets: Query<&GizmoTarget>,
    mut picking_settings: ResMut<PickingPlugin>,
) {
    // Picking is disabled when any of the gizmos is focused or active.
    picking_settings.is_enabled = gizmo_targets
        .iter()
        .all(|target| !target.is_focused() && !target.is_active());
}

pub fn get_top_pickable_entity(
    picked_entity: Entity,
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
) -> Entity {
    // Ascend parent chain to get root selectable object
    let top_entity = if let Ok((e, parent)) = pick_collectors.get(picked_entity) {
        let mut top_entity_check = Ok((e, parent));
        let mut top_parent = e;

        while let Ok((e, parent)) = top_entity_check {
            let next_parent = parent.get();
            top_entity_check = pick_collectors.get(next_parent);
            top_parent = next_parent;
        }
        top_parent
    } else {
        picked_entity
    };
    top_entity
}

pub fn pick_self_select_air_deselect(
    pointer: Single<&PointerInteraction>,
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
    mut interactables: Query<&mut PickSelected>,
    mouse: ResMut<ButtonInput<MouseButton>>,
) {
    let mouse_pressed = mouse.just_pressed(MouseButton::Left);
    let Some((e, hit)) = pointer.first() else {
        // remove selection if air is clicked
        if mouse_pressed {
            for mut selected in &mut interactables {
                selected.0 = false;
            }
         }
        return;
    };


    if mouse_pressed {
        // remove all preivous selection if hiting something that is in the world(not a window)
        // TODO: If multi-select is added at some point, this will need to be changed to check for that.
        if hit.position.is_some() {
            if mouse_pressed {
                for mut selected in &mut interactables {
                    selected.0 = false;
                }
             }
        }
        let Ok(mut picked) = interactables.get_mut(get_top_pickable_entity(*e, pick_collectors)) else {
            return
        };

        picked.0 = true;
    }
}

/// behaviour for what happens when stuff is clicked on
pub fn pick_self_select_deselect(
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
    mut interactables: Query<&mut PickSelected>,
    pointer: Single<&PointerInteraction>,
    mouse: ResMut<ButtonInput<MouseButton>>,
) {

    let Some((e, _hit)) = pointer.first() else {
        return;
    };
    let Ok(mut picked) = interactables.get_mut(get_top_pickable_entity(*e, pick_collectors)) else {
        return
    };

    if mouse.just_pressed(MouseButton::Left) {
        if picked.0 {
            picked.0 = false;
        } else {
            picked.0 = true;
        }
    }

}


// /// behaviour of what happens when stuff is clicked on. 
// pub fn picking_interaction_effects(
//     mut interactables: Query<&mut PickSelected>,
//     pointer: Single<&PointerInteraction>
// ) {
//     let Some((e, hit)) = pointer.first() else {
//         return;
//     };

//     let Ok(mut picked) = interactables.get_mut(*e) else {
//         return
//     };

//     if picked.0 {
//         picked.0 = false;
//     } else {
//         picked.0 = true;
//     }
// }

// /// effects on things that are clicked on
// pub fn picking_click_effects(
//     mut commands: Commands,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut clickables: Query<(Entity, 
//         Option<&Part>, 
//         &PickSelected), Changed<PickSelected>>,
//     structures: Query<(Entity, &Part)>,
//     joints: Query<&JointFlag>,
//     // mut rigid_bodies: Query<&mut RigidBodyFlag>,
//     // mut outlines: Query<&mut OutlineVolume>,
//     // mut tool_mode: ResMut<NextState<BuildToolMode>>,
//     // mut assembling_target_structure: Query<(&StructureFlag, &AssemblingTarget)>
// ) {
//     // Continuously update entities based on their picking state

//     for (e, structure, selected) in &mut clickables {
//         if **selected {
//             let mut entity_cmd = commands.entity(e);

//             // entity_cmd.insert(GizmoTarget::default());

//             if keys.pressed(KeyCode::ShiftLeft) {
//                 entity_cmd.insert(AssemblingTarget);
//             } else if let Ok(joint) = joints.get(e) {
//                 //if let Ok(joint) = joints.get(e) {
//                 entity_cmd.insert(AttachCandidate {
//                     attempt_target: Some(joint.parent),
//                 });
//             } else {
//                 entity_cmd.insert(GizmoTarget::default());
//             }

//             //}
//             // if let Some(structure) = structure {

//             //     //if structures.iter().find(|(e_target, n)| n.name == structure.name)
//             //     // if structures.iter().any(||n.name == structure.name) {
//             //     //     entity_cmd.insert(AttachCandidate {
//             //     //         attempt_target: Some(e)
//             //     //     });
//             //     // }
//             // }
//         } else {
//             let mut entity_cmd = commands.entity(e);

//             entity_cmd.remove::<GizmoTarget>();
//             // entity_cmd.remove::<AssemblingTarget>();
//             // let _ = rigid_bodies.get_mut(e).map(|mut rigid_body| *rigid_body = RigidBodyFlag::Dynamic);
//             // if let Ok(mut outline) = outlines.get_mut(e) {
//             //     outline.visible = false;

//             // }
//         }
//     }
// }

pub fn make_models_pickable(
    mut commands: Commands,
    models_query: Query<Entity, (
        With<Mesh3d>,
        Without<RayCastPickable>
    )>,
) {
    for e in models_query.iter() {
        commands.entity(e).insert((
            RayCastPickable,
            // OutlineVolume {
            //     visible: false,
            //     colour: Color::Srgba(Srgba::GREEN),
            //     width: 2.0,
            // },
        ));
    }
}
