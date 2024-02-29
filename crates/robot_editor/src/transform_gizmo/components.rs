use std::time::Duration;

use bevy::prelude::*;



#[derive(Component, Reflect, Default)]
pub struct Widget;

#[derive(Component, Clone, Copy, Reflect, Default)]
#[reflect(Component)]
pub enum SelectionMode {
    #[default]
    Selecting,
    Clicking,
}

#[derive(Component, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct LastMouseInteraction {
    pub mouse_pos: Vec2,

    pub time_of_interaction: f64,
    //pub hold_duration: Option<f32>,
}

/// flag + direction of tug, when dragged, things with tug pull their widget in this components direction.
#[derive(Component, Deref, Reflect, Default)]
#[reflect(Component)]
pub struct Tug(Vec3);

impl Tug {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

/// flag + axis of ring, when dragged, things will rotate by their widget in this component's axis
#[derive(Component)]
pub struct Ring {
    pub axis: Vec3,
}

impl Ring {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            axis: Vec3::new(x, y, z),
        }
    }
}

// Collects commands from widgets, applies them to bound widget command reciever
#[derive(Component)]
pub struct TransformWidget; /*{
                                pub bound_entity: Entity,
                            }*/

#[derive(Component)]
pub struct TransformWidgetMarker {
    pub transform_widget_entity: Entity,
    /// entity to be modified by transform widget
    pub entity_to_transform: Entity,
}
