use std::{f32::consts::PI, ops::Mul};

use bevy::{prelude::*, render::view::RenderLayers, transform::commands, utils::HashMap};
use bevy_mod_raycast::{immediate::Raycast, CursorRay};
use bevy_serialization_extras::prelude::link::JointFlag;

use crate::{
    components::GizmoFocused, shaders::neon_glow::NeonGlowMaterial, ui::{get_first_hit_with, get_first_hit_with_mut, BuildToolMode}
};

use super::components::{Ring, TransformWidget, TransformWidgetMarker, Tug, Widget};

const TRANSFORM_GIZMO_ACTIVE: BuildToolMode = BuildToolMode::GizmoMode;
const GIZMO_CAMERA_LAYER: u8 = 1;


/// marks this camera as a gizmo cam which mirrors the camera atteched to the set mirrored_camera
#[derive(Component, Deref)]
pub struct CameraMirrors(Entity);
pub fn spawn_gizmo_when_needed(
    //models_without_widget: Query<(Entity, &Transform, &GizmoFocused), (With<Handle<Mesh>>, Without<Widget>)>,
    models_marked_for_gizmo: Query<(Entity, &GizmoFocused)>,
    spawned_gizmos: Query<&TransformWidget>,
    //widgets_to_despawn: Query<(Entity, &TransformWidgetMarker), Without<Selected>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_material: ResMut<Assets<NeonGlowMaterial>>,
    tool_mode: Res<BuildToolMode>,
) {
    //spawn transform widgets on selected entities if there is no transform gizmo in the world
    if models_marked_for_gizmo.iter().len() > 0
        && spawned_gizmos.iter().len() == 0
        && *tool_mode == TRANSFORM_GIZMO_ACTIVE
    {
        let cube_size = 0.3;

        let dist = 1.0;

        //let cube_mesh = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));
        let cube_x_mesh = meshes.add(Cuboid::new(cube_size * 3.0, cube_size, cube_size));
        let cube_y_mesh = meshes.add(Cuboid::new(cube_size, cube_size * 3.0, cube_size));
        let cube_z_mesh = meshes.add(Cuboid::new(cube_size, cube_size, cube_size * 3.0));

        let disc_mesh = meshes.add(
            Torus {
                minor_radius: 0.1,
                major_radius: dist,
                // radius: dist,
                // ring_radius: 0.1,
                // subdivisions_segments: 10,
                // subdivisions_sides: 10,
            },
        );
        let transform_widget = commands
            .spawn((
                // spawn out of sight, and let different system correct position
                SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
                TransformWidget,
                Name::new("Transform Widget"),
                RenderLayers::layer(GIZMO_CAMERA_LAYER)
            )).id();
        // spawn edit widget, x = red, y = green, z = blue

        // some these are probably wrong and will need tweaking...
        let (s, l) = (0.8, 0.6);
        let y_tug = commands
            .spawn((
                MaterialMeshBundle {
                    mesh: cube_y_mesh.clone(),
                    material: gizmo_material.add(NeonGlowMaterial::from(Color::hsl(120.0, s, l))),
                    transform: Transform::from_translation(Vec3::new(0.0, dist, 0.0)),
                    ..default()
                },
                Name::new("y_tug"),
                Widget,
                Tug::new(0.0, 1.0, 0.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        let y_tug_negative = commands
            .spawn((
                MaterialMeshBundle {
                    mesh: cube_y_mesh.clone(),
                    material: gizmo_material.add(NeonGlowMaterial::from(Color::hsl(120.0, s, l))),
                    transform: Transform::from_translation(Vec3::new(0.0, -dist, 0.0)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("y_tug_negative"),
                Widget,
                Tug::new(0.0, 1.0, 0.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        let x_tug = commands
            .spawn((
                PbrBundle {
                    mesh: cube_x_mesh.clone(),
                    material: materials.add(Color::RED),
                    transform: Transform::from_translation(Vec3::new(dist, 0.0, 0.0)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("x_tug"),
                Widget,
                Tug::new(1.0, 0.0, 0.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        let x_tug_negative = commands
            .spawn((
                PbrBundle {
                    mesh: cube_x_mesh.clone(),
                    material: materials.add(Color::RED),
                    transform: Transform::from_translation(Vec3::new(-dist, 0.0, 0.0)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("x_tug_negative"),
                Widget,
                Tug::new(1.0, 0.0, 0.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        let z_tug = commands
            .spawn((
                PbrBundle {
                    mesh: cube_z_mesh.clone(),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, dist)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("z_tug"),
                Widget,
                Tug::new(0.0, 0.0, 1.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        let z_tug_negative = commands
            .spawn((
                PbrBundle {
                    mesh: cube_z_mesh.clone(),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, -dist)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("z_tug_negative"),
                Widget,
                Tug::new(0.0, 0.0, 1.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        // discs

        // side ring
        let y_axis_ring = commands
            .spawn((
                PbrBundle {
                    mesh: disc_mesh.clone(),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("y_axis_ring"),
                Widget,
                //y_ring_flag,
                Ring::new(0.0, 1.0, 0.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER),
            ))
            .id();
        // top ring
        let z_axis_ring = commands
            .spawn((
                PbrBundle {
                    mesh: disc_mesh.clone(),
                    material: materials.add(Color::BLUE),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_x(PI / 2.0)),
                    ..default()
                },
                //MakeSelectableBundle::default(),
                Name::new("z_axis_ring"),
                Widget,
                //z_ring_flag,
                Ring::new(0.0, 0.0, 1.0),
                RenderLayers::layer(GIZMO_CAMERA_LAYER)
            ))
            .id();

            // set widget root transform to equal model the widget is spawning around
            commands.entity(transform_widget)
            .add_child(y_tug)
            .add_child(y_tug_negative)
            .add_child(x_tug)
            .add_child(x_tug_negative)
            .add_child(z_tug)
            .add_child(z_tug_negative)
            .add_child(y_axis_ring)
            .add_child(z_axis_ring)
            ;
    }
}


pub fn drag_tugs_with_mouse(
    cursor_ray: Res<CursorRay>, 
    raycast: Raycast,
    tugs: Query<(&Transform, &Parent, &Tug)>,
    mut gizmo_focused: Query<&mut Transform, (With<GizmoFocused>, Without<Parent>, Without<Tug>)>,
    mouse: Res<ButtonInput<MouseButton>>,

) {
    if mouse.pressed(MouseButton::Left) {
        if let Some((_, data, (tug_trans, parent, tug))) = get_first_hit_with(cursor_ray, raycast, &tugs) {
            for mut trans in gizmo_focused.iter_mut() {
                if tug.x > 0.0 {
                    trans.translation.x = data.position().x - tug_trans.translation.x
                }
                if tug.y > 0.0 {
                    trans.translation.y = data.position().y - tug_trans.translation.y
        
                }
                if tug.z > 0.0 {
                    trans.translation.z = data.position().z - tug_trans.translation.z
        
                }
            }
            // if let Ok(mut parent_trans) = trans.get_mut(**parent) {
            //     if tug.x > 0.0 {
            //         parent_trans.translation.x = data.position().x - tug_trans.translation.x
            //     }
            //     if tug.y > 0.0 {
            //         parent_trans.translation.y = data.position().y - tug_trans.translation.y
        
            //     }
            //     if tug.z > 0.0 {
            //         parent_trans.translation.z = data.position().z - tug_trans.translation.z
        
            //     }
            // }

        }
    } 
}

pub fn collect_tug_forces() {}

/// get all of the models marked for transform gizmo, and set transform gizmo to be the average position between all of them.
pub fn average_gizmo_position(
    gizmo_marked_models: Query<&Transform, (With<GizmoFocused>, Without<TransformWidget>)>,
    mut transform_gizmo_pos: Query<&mut Transform, With<TransformWidget>>,
) {
    let mut collected_pos = Vec3::ZERO;
    let mut pos_count = 0.0;
    for trans in gizmo_marked_models.iter() {
        collected_pos += trans.translation;
        pos_count += 1.0;
    }
    let averaged_pos = collected_pos / pos_count;

    for mut trans in transform_gizmo_pos.iter_mut() {
        (*trans).translation = averaged_pos;
    }
}


pub fn spawn_gizmo_rendering_camera(
    unfocused_gizmos: Query<Entity, With<Widget>>,
    cameras: Query<Entity, (With<Camera3d>, Without<CameraMirrors>)>,
    mirror_cameras: Query<Entity, With<CameraMirrors>>,
    mut commands: Commands,
) {

    //FIXME: this will break if there are multiple cameras.
    if let Some(main_camera) = cameras.iter().last() {
        if unfocused_gizmos.iter().len() > 0 && mirror_cameras.iter().len() <= 0 {
            commands.spawn(
                (
                    Camera3dBundle {
                        transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
                        camera_3d: Camera3d {
                            //clear_color: ClearColorConfig::None,
                            ..default()
                        },
                        camera: Camera {
                            // renders after / on top of the main camera
                            order: 1,
                            ..default()
                        },
                        ..default()
                    },
                    // set to render layer 1 to make camera see models on render layer 1
                    RenderLayers::layer(GIZMO_CAMERA_LAYER),
                    CameraMirrors(main_camera)
                    )
            )
            ;
            }
    }

}

/// makes gizmo camera follow gizmo
pub fn align_gizmo_camera_to_marker(
    mut mirror_cameras: Query<(&mut Transform, &CameraMirrors)>,
    non_mirror_cameras: Query<&Transform, (With<Camera3d>, Without<CameraMirrors>)>,

) {
    for (mut trans, mirrored_e) in mirror_cameras.iter_mut() {
        if let Ok(mirrored_trans) = non_mirror_cameras.get(**mirrored_e) {
            *trans = *mirrored_trans;
        }
    }
}

pub fn despawn_gizmo_rendering_camera(
    transform_gizmos: Query<&TransformWidget>,
    mirror_cameras: Query<Entity, With<CameraMirrors>>,
    mut commands: Commands,
) {
    if transform_gizmos.iter().len() <= 0 {
        for e in mirror_cameras.iter() {
            commands.entity(e)
            .despawn()
        }
    }
}

pub fn despawn_gizmo_when_no_targets(
    gizmo_marked_models: Query<(Entity, &GizmoFocused)>,
    transform_gizmos: Query<Entity, With<TransformWidget>>,
    tool_mode: Res<BuildToolMode>,
    mut commands: Commands,
) {
    if gizmo_marked_models.iter().len() <= 0 || *tool_mode != TRANSFORM_GIZMO_ACTIVE {
        for e in transform_gizmos.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
}

/// mark/unmark model for transform gizmo on click
pub fn gizmo_mark_on_click(
    cursor_ray: Res<CursorRay>,
    raycast: Raycast,
    mut tool_mode: ResMut<BuildToolMode>,
    gizmoable: Query<&Transform>,
    gizmo_filter: Query<Entity, With<Widget>>,
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    things_with_gizmo: Query<&GizmoFocused>,
) {
    if *tool_mode == BuildToolMode::GizmoMode {
        if mouse.just_pressed(MouseButton::Left) {
            if let Some((e, ..)) = get_first_hit_with(cursor_ray, raycast, &gizmoable) {
                if gizmo_filter.contains(e) == false {
                    //println!("selecting for gizmo");
                    if things_with_gizmo.contains(e) {
                        commands.entity(e).remove::<GizmoFocused>();
                        //*tool_mode = BuildToolMode::SelectorMode
                    } else {
                        commands.entity(e).insert(GizmoFocused);
                        //*tool_mode = BuildToolMode::GizmoMode
                    }
                }

            }
        }
    }
}
