use bevy::{ecs::system::Resource, reflect::Reflect};
use strum_macros::{Display, EnumIter};

#[derive(Resource, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum BuildToolMode {
    GizmoMode,
    PlacerMode,
    SelectorMode,
    //AttachMode,
    EditerMode,
}
