pub mod components;
pub mod plugins;
mod systems;

pub use bevy_ecs::prelude::*;
use bevy_hierarchy::Parent;
use bevy_picking::mesh_picking::ray_cast::{RayCastSettings, RayCastVisibility};
use bevy_state::state::States;
use components::PickCollector;

pub const DONT_EXIT_EARLY: RayCastSettings<'static> = RayCastSettings {
    visibility: RayCastVisibility::VisibleInView,
    filter: &|_| true,
    early_exit_test: &|_| false,
};

pub const EXIT_EARLY: RayCastSettings = RayCastSettings {
    visibility: RayCastVisibility::Any,
    filter: &|_| true,
    early_exit_test: &|_| false,
};



/// Different modes for handling how "selecting" stuff works
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PickMode {
    
    #[default]
    /// pick to select, pick to deselect.
    PickSelfSelectDeselect,
    /// pick to select, click on air to deselect
    PickSelfSelectAirDeselect,
}

pub fn get_top_pickable_entity(
    picked_entity: Entity,
    pick_collectors: Query<(Entity, &Parent), With<PickCollector>>,
) -> Entity {
    // Ascend parent chain to get root selectable object
    let top_entity = if let Ok((e, parent)) = pick_collectors.get(picked_entity) {
        let mut top_entity_check = Ok((e, parent));
        let mut top_parent = e;

        while let Ok((_, parent)) = top_entity_check {
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