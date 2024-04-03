use bevy::{
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData},
    prelude::*,
};
use bevy_mod_raycast::{
    immediate::{Raycast, RaycastSettings, RaycastVisibility},
    primitives::IntersectionData,
    CursorRay,
};

use super::resources::MouseOverWindow;

pub const DONT_EXIT_EARLY: RaycastSettings = RaycastSettings {
    visibility: RaycastVisibility::MustBeVisibleAndInView,
    filter: &|_| true,
    early_exit_test: &|_| false,
};

/// Get hit data for first hit entity not in hit criteria
pub fn get_first_hit_without<'a, T: ReadOnlyQueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a Query<T, F>,
) -> Option<(Entity, IntersectionData)> {
    let first_hit = hit_list?
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
        .nth(0)?;

    //let query_data = hit_match_criteria.get(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone()))
}
/// Gets all hit data of entities clicked on by mouse.  
pub fn cursor_ray_hititer<'a>(
    cursor_ray: &'a Res<CursorRay>,
    raycast: &'a mut Raycast,
    mouse_over_window: &'a Res<MouseOverWindow>,
) -> Option<std::slice::Iter<'a, (Entity, IntersectionData)>> {
    if ***mouse_over_window {
        return None;
    }
    let ray = (***cursor_ray)?;
    let hit_list = raycast.cast_ray(ray, &DONT_EXIT_EARLY).iter();
    Some(hit_list)
}

/// Get hit data for first hit entity in hit criteria + query data(immutable)
pub fn get_first_hit_with<'a, T: ReadOnlyQueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a Query<T, F>,
) -> Option<(Entity, IntersectionData, T::Item<'a>)>
where
{
    let first_hit = hit_list?
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
        .nth(0)?;

    let query_data = hit_match_criteria.get(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}

/// Get hit data for first hit entity in hit criteria + query data(mutable)
pub fn get_first_hit_with_mut<'a, T: QueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a mut Query<T, F>,
) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
    let first_hit = hit_list?
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
        .nth(0)?;

    let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}

/// Get hit data for first hit entity in hit criteria(with mutable query input)
pub fn get_first_hit_without_mut<'a, T: QueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a mut Query<T, F>,
) -> Option<(Entity, IntersectionData)> {
    let first_hit = hit_list?
        .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
        .nth(0)?;

    //println!("first hit is {:#?}", first_hit);
    //let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone()))
}
