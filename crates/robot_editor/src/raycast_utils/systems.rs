use bevy::{picking::pointer::PointerInteraction, prelude::*};
use bevy_picking::mesh_picking::ray_cast::RayMeshHit;

use super::resources::{CursorRayHits, RayCastDebugMode};



// pub fn update_raycast_sources(
//     mut raycasts: Query<(&mut RaycastSource, &Transform)>,
//     mut raycast: MeshRayCast,
//     mouse_over_window: Res<MouseOverWindow>,
//     debug_mode: Res<RayCastDebugMode>,
//     mut gizmos: Gizmos,
// ) {
//     for (mut ray_source, transform) in raycasts.iter_mut() {
//         ray_source.hits = ray_hit_iter(
//             &ray_source.ray,
//             &mut raycast,
//             &mouse_over_window,
//             &debug_mode,
//             &mut gizmos,
//         )
//         .unwrap_or_default()
//         .map(|n| n.clone())
//         .collect::<Vec<_>>();
//     }
// }

///
// pub fn update_cursor_ray_hits(
//     cursor_ray: Single<&PointerInteraction>,
//     mut cursor_ray_hits: ResMut<CursorRayHits>,
//     mut raycast: MeshRayCast,
//     mouse_over_window: Res<MouseOverWindow>,
//     debug_mode: Res<RayCastDebugMode>,
//     mut gizmos: Gizmos,
// ) {
//     //println!("cursor ray: {:#?}", cursor_ray.0);
//     let hit_iter = ray_hit_iter(
//         &cursor_ray,
//         &mut raycast,
//         &mouse_over_window,
//         &debug_mode,
//         &mut gizmos,
//     )
//     .unwrap_or_default()
//     .map(|n| n.clone())
//     .collect::<Vec<_>>();

//     **cursor_ray_hits = hit_iter
// }

// /// Gets all hit data of entities clicked on by mouse.
// pub fn ray_hit_iter<'a>(
//     ray: &'a Option<Ray3d>,
//     raycast: &'a mut MeshRayCast,
//     debug_mode: &'a Res<RayCastDebugMode>,
//     gizmos: &'a mut Gizmos,
// ) -> Option<std::slice::Iter<'a, (Entity, RayMeshHit)>> {
//     if ***mouse_over_window {
//         return None;
//     }
//     let Some(ray) = *ray else { return None };
//     if ***debug_mode {
//         warn!("re-add debug ray visualization");
        
//         //raycast.debug_cast_ray(ray, &DONT_EXIT_EARLY, gizmos).iter()
//     } 
    
//     else {
//     };
//     let hit_list = raycast.cast_ray(ray, &DONT_EXIT_EARLY).iter();

//     Some(hit_list)
// }
