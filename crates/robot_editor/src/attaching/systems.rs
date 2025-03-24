use bevy::prelude::*;
use bevy_serialization_extras::prelude::{link::JointFlag, Dynamics, JointAxesMaskWrapper, JointInfo, JointLimitWrapper, JointMotorWrapper};

use super::components::*;
use crate::placing::components::Placer;

// /// gets rid of placers if current mode is not placermode
// // pub fn delete_attach_candidates(
// //     tool_mode: ResMut<State<BuildToolMode>>,
// //     placers: Query<Entity, With<AttachCandidate>>,
// //     mut commands: Commands,
// // ) {
// //     if *tool_mode != BuildToolMode::EditerMode {
// //         for e in placers.iter() {
// //             commands.entity(e).despawn()
// //         }
// //     }
// // }

/// attach candidate if its been clicked on something
pub fn confirm_attachment(
    candidates: Query<(Entity, &Transform, &AttachCandidate)>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for (e, transform, candidate) in candidates.iter() {
            let Some(target) = candidate.attempt_target else {
                return;
            };

            commands.entity(e).insert(JointFlag {
                parent: target,
                //TODO: add default impl back on next update
                joint: JointInfo {
                    local_frame1: Transform::default(),
                    local_frame2: transform.clone(),
                    limit: JointLimitWrapper::default(),
                    dynamics: Dynamics::default(),
                    limit_axes: JointAxesMaskWrapper::default(),
                    locked_axes: JointAxesMaskWrapper::default(),
                    motor_axes: JointAxesMaskWrapper::default(),
                    contacts_enabled: false,
                    coupled_axes: JointAxesMaskWrapper::default(),
                    motors: [
                        JointMotorWrapper::default(),
                        JointMotorWrapper::default(),
                        JointMotorWrapper::default(),
                        JointMotorWrapper::default(),
                        JointMotorWrapper::default(),
                        JointMotorWrapper::default()
                    ],
                    enabled: true
                    
                }

            });
            commands.entity(e).remove::<AttachCandidate>();
        }
    }
}

/// switch to attach move to placer
pub fn switch_to_attach_from_placer(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut placers: Query<(Entity, Option<&mut AttachCandidate>), With<Placer>>,
    joints: Query<&JointFlag>,
    mouse: Res<ButtonInput<MouseButton>>,
    // hits: ResMut<CursorRayHits>,
    mut commands: Commands,
) {
    // if keys.pressed(KeyCode::ShiftLeft) {
    //     let Ok((e, current_target, ..)) = placers.get_single_mut().inspect_err(|err| {
    //         warn!(
    //             "switching attacher mode only works with 1 placer: Actual error: {:#}",
    //             err
    //         )
    //     }) else {
    //         return;
    //     };

    //     if let Some((target, hit)) = hits.first_hit_after(&e) {
    //         if let Some(mut current_target) = current_target {
    //             current_target.attempt_target = Some(*target)
    //         } else {
    //             commands.entity(e).insert(AttachCandidate {
    //                 attempt_target: Some(*target),
    //             });
    //         }
    //     }
    //     // for (e, current_target) in placers.iter() {
    //     //     if let Some((target,..)) = hits.first_wi(&placers) {
    //     //         if let Some(current_target) = current_target {

    //     //         }
    //     //         commands.entity(e).insert(AttachCandidate {
    //     //             attempt_target: Some(target)
    //     //         });
    //     //     }
    //     // }
    // } else {
    //     let Ok((e, current_target, ..)) = placers.get_single_mut() else {
    //         return;
    //     };

    //     //  let Some(current_target) = current_target else {return;};

    //     // remove attach candidates if they aren't attached to anything
    //     if joints.get(e).is_err() {
    //         commands.entity(e).remove::<AttachCandidate>();
    //     }
    // }
}

// /// manages gizmos associated with attacher
// pub fn manage_attacher_position() {}

// /// confirms attacher target for attacher
// pub fn confirm_attacher_target() {}
