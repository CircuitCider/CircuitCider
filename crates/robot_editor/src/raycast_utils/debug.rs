//! Debug utils for raycasting/testing

use std::ptr::from_ref;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_mod_raycast::{immediate::Raycast, primitives::ray_from_screenspace, CursorRay};
use egui::{Color32, RichText};

//use crate::selection_behaviour::components::Grabbed;

use super::systems::DONT_EXIT_EARLY;

/// shoot a ray from cursor to target transform


/// gives useful info from raycast.
pub fn debug_mouse_info(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    let origin_rgba = Color32::RED;
    let direction_rgba = Color32::BLUE;
    let ray_intersection_rgba = Color32::GREEN;

    //let b: Color = Color::rgba_from_array(origin_rgba.to_array().map(|x| x as f32));
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Mouse Ray info").show(context.get_mut(), |ui| {
            if let Some(ray) = **cursor_ray {

                ui.horizontal(|ui| {
                    ui.label(RichText::new("origin: ").color(origin_rgba));
                    ui.label(format!("{:#}", ray.origin));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("direction: ").color(direction_rgba));
                    ui.label(format!("{:#}", *ray.direction));

                });

                gizmos.line(ray.origin, *ray.direction, Color::rgba_from_array(direction_rgba.to_array().map(|x| x as f32)));

                let hits = raycast.cast_ray(ray, &DONT_EXIT_EARLY);
                ui.label(RichText::new("Mouse ray intersection").color(ray_intersection_rgba));
                //gizmos.primitive_3d(primitive, position, angle, color);
                if let Some((.., hit)) = hits.iter().next() {
                    let color = Color::rgba_from_array(ray_intersection_rgba.to_array().map(|x| x as f32));
                    gizmos.ray(hit.position(), hit.normal(), color);
                    gizmos.circle(
                        hit.position(),
                        Direction3d::new_unchecked(hit.normal().normalize()),
                        0.1,
                        color,
                    );
                }
                // for (is_first, intersection) in hits
                //     .iter()
                //     .map(|i| i.1.clone())
                //     .enumerate()
                //     .map(|(i, hit)| (i == 0, hit))
                // {
                //     let (x, y, z) = (intersection.position().x, intersection.position().y, intersection.position().z);
                //     ui.label(
                //         format!("{:#}", intersection.position())
                //     );
                //     let color = match is_first {
                //         true => Color::rgba_from_array(ray_intersection_rgba.to_array().map(|x| x as f32)),
                //         false => Color::PINK,
                //     };
                //     gizmos.ray(intersection.position(), intersection.normal(), color);
                //     gizmos.circle(
                //         intersection.position(),
                //         Direction3d::new_unchecked(intersection.normal().normalize()),
                //         0.1,
                //         color,
                //     );
                // }
            }
        });
    }
}