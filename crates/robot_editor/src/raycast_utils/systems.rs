use bevy::{ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData}, prelude::*};
use bevy_mod_raycast::{immediate::{Raycast, RaycastSettings, RaycastVisibility}, primitives::IntersectionData, CursorRay};

use super::resources::MouseOverWindow;


const DONT_EXIT_EARLY: RaycastSettings = RaycastSettings {
    visibility: RaycastVisibility::MustBeVisibleAndInView,
    filter: &|_| true,
    early_exit_test: &|_| false,
};

pub fn get_first_hit_without<'a, T: ReadOnlyQueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a Query<T>,
) -> Option<(Entity, IntersectionData, T::Item<'a>)> {

    let first_hit = hit_list?
    .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
    .nth(0)?;
    
    let query_data = hit_match_criteria.get(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}

pub fn get_first_hit_without_mut<'a, T: QueryData, F: QueryFilter>(
    hit_list: Option<std::slice::Iter<'a, (Entity, IntersectionData)>>,
    hit_match_criteria: &'a mut Query<T, F>,
) -> Option<(Entity, IntersectionData, T::Item<'a>)> {

    let first_hit = hit_list?
    .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
    .nth(0)?;
    
    let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}

pub fn cursor_ray_hititer<'a>(
    cursor_ray: Res<CursorRay>,
    raycast: &'a mut Raycast,
    mouse_over_window: Res<MouseOverWindow>

) -> Option<std::slice::Iter<'a, (Entity, IntersectionData)>>
{
    if **mouse_over_window {
        return None
    }
    let ray = (**cursor_ray)?;
    let hit_list = raycast
        .cast_ray(ray, &DONT_EXIT_EARLY)
        .iter()
        ;
    Some(hit_list)
}
/// gets first hit with raycast from cursor which matches a given query.
pub fn get_first_hit_with<'a, T: ReadOnlyQueryData, F: QueryFilter>(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    hit_match_criteria: &'a Query<T, F>,
    mouse_over_window: Res<MouseOverWindow>
) -> Option<(Entity, IntersectionData, T::Item<'a>)> 
    where
{
    let first_hit = cursor_ray_hititer(cursor_ray, &mut raycast, mouse_over_window)?
    .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
    .nth(0)?;
    
    let query_data = hit_match_criteria.get(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}

/// get first hit entity that matches a query, and return the entity, mutable query data, and intersection data
pub fn get_first_hit_with_mut<'a, T: QueryData, F: QueryFilter>(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    hit_match_criteria: &'a mut Query<'_, '_, T, F>,
    mouse_over_window: Res<MouseOverWindow>
) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
    let ray = (**cursor_ray)?;

    if **mouse_over_window {
        return None
    }

    let first_hit = cursor_ray_hititer(cursor_ray, &mut raycast, mouse_over_window)?
    .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
    .nth(0)?;

    let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

    Some((first_hit.0, first_hit.1.clone(), query_data))
}