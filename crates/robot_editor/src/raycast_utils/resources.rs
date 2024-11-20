use bevy::{
    ecs::{
        entity::Entity,
        query::{QueryData, QueryFilter, ReadOnlyQueryData},
        system::{Query, Resource},
    },
    math::Ray3d,
    prelude::{Component, Deref, DerefMut},
    reflect::Reflect,
};
use bevy_mod_raycast::primitives::IntersectionData;

/// weather mouse is over window or not.
#[derive(Resource, Reflect, Deref, DerefMut, Default)]
pub struct MouseOverWindow(bool);



/// collection of things that [`CursorRay`] hit.
///
/// TODO: Give hit filter functions to this as an impl. very clunky to use this ATM.
#[derive(Resource, Default, Deref, DerefMut, Reflect)]
pub struct CursorRayHits(pub Vec<(Entity, IntersectionData)>);

// #[derive(Resource, Default, Deref)]
// pub struct RayCasts(pub Option<Ray3d>);

/// this entity shoots rays
#[derive(Component, Default)]
pub struct RaycastSource {
    pub ray: Option<Ray3d>,
    pub hits: Vec<(Entity, IntersectionData)>,
}

impl RaycastSource {
    /// Get hit data for first hit entity not in hit criteria
    pub fn first_without<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a Query<T, F>,
    ) -> Option<(Entity, IntersectionData)> {
        let first_hit = self
            .hits
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
            .nth(0)?;

        //let query_data = hit_match_criteria.get(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone()))
    }
    pub fn first_with<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a Query<T, F>,
    ) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
        let first_hit = self
            .hits
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
            .nth(0)?;

        let query_data = hit_match_criteria.get(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone(), query_data))
    }
    /// Get hit data for first hit entity in hit criteria + query data(mutable)
    pub fn first_with_mut<'a, T: QueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a mut Query<T, F>,
    ) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
        let first_hit = self
            .hits
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
            .nth(0)?;

        let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone(), query_data))
    }
    /// Get hit data for first hit entity in hit criteria(with mutable query input)
    pub fn first_without_mut<'a, T: QueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a mut Query<T, F>,
    ) -> Option<(Entity, IntersectionData)> {
        let first_hit = self
            .hits
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
            .nth(0)?;

        //println!("first hit is {:#?}", first_hit);
        //let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone()))
    }
}

/// weather shot rays should be gizmo rendered.
#[derive(Resource, Deref, DerefMut, Reflect, Default)]
pub struct RayCastDebugMode(pub bool);

impl CursorRayHits {
    /// Get hit data for first hit entity not in hit criteria
    pub fn first_without<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a Query<T, F>,
    ) -> Option<(Entity, IntersectionData)> {
        let first_hit = self
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
            .nth(0)?;

        //let query_data = hit_match_criteria.get(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone()))
    }
    pub fn first_with<'a, T: ReadOnlyQueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a Query<T, F>,
    ) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
        let first_hit = self
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
            .nth(0)?;

        let query_data = hit_match_criteria.get(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone(), query_data))
    }
    /// Get hit data for first hit entity in hit criteria + query data(mutable)
    pub fn first_with_mut<'a, T: QueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a mut Query<T, F>,
    ) -> Option<(Entity, IntersectionData, T::Item<'a>)> {
        let first_hit = self
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == true)
            .nth(0)?;

        let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone(), query_data))
    }
    /// Get hit data for first hit entity in hit criteria(with mutable query input)
    pub fn first_without_mut<'a, T: QueryData, F: QueryFilter>(
        &self,
        hit_match_criteria: &'a mut Query<T, F>,
    ) -> Option<(Entity, IntersectionData)> {
        let first_hit = self
            .iter()
            .filter(|(e, ..)| hit_match_criteria.contains(e.clone()) == false)
            .nth(0)?;

        //println!("first hit is {:#?}", first_hit);
        //let query_data = hit_match_criteria.get_mut(first_hit.0).ok()?;

        Some((first_hit.0, first_hit.1.clone()))
    }
    /// checks to see if the given entity was hit, returns None if not.
    pub fn first_hit(
        &self,
        entity: &Entity,
    ) -> Option<&(Entity, IntersectionData)> {
        self.iter().find(|(e, ..)| e == entity)
    }
    /// gets the first hit after the given entity, returns if there is none.
    pub fn first_hit_after(
        &self,
        entity: &Entity
    ) -> Option<(&Entity, &IntersectionData)> { 

        for (i, (current, current_hit)) in self.iter().enumerate()
        //overflow protection
        .skip(0) {
            if i == 0 {
                continue
            }
            if let Some((previous, hit)) = self.get(i - 1) {
                if previous == entity {
                    return Some((current, current_hit))
                }
            }
        }
        None
        // let hit = self.iter().enumerate()
        // .find(|( i, _)| {
        //     // subtracting unassigned types can cause underflow. this stops that panic
        //     // trying to compare &usize and &usize makes mold crash, so this needs a copy ;(
        //     if *i != 0 {
        //         false
        //     } else {
        //         if let Some((e, _)) = self.iter().nth(i - 1) {
        //             e == entity
        //         } else {
        //             false
        //         }
        //     }

        //     }
        // )
        // .map(|(_, (e, hit))| (e, hit));
        // hit
    }
}
