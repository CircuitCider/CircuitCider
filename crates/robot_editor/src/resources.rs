use bevy::{ecs::system::Resource, prelude::KeyCode, reflect::Reflect};
use strum_macros::{Display, EnumIter};

#[derive(Resource, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum BuildToolMode {
    GizmoMode,
    PlacerMode,
    SelectorMode,
    //AttachMode,
    EditerMode,
}

#[derive(Resource, Reflect)]
pub struct RobotControls {
    pub target_speed: f32,

    pub leftward_key: KeyCode,
    pub rightward_key: KeyCode,
    pub forward_key: KeyCode,
    pub backward_key: KeyCode,

    pub freeze_key: KeyCode,
    pub unfreeze_key: KeyCode,
}

impl Default for RobotControls {
    fn default() -> Self {
        RobotControls {
            target_speed: 20.0,
            leftward_key: KeyCode::ArrowLeft,
            rightward_key: KeyCode::ArrowRight,
            forward_key: KeyCode::ArrowUp,
            backward_key: KeyCode::ArrowDown,
            freeze_key: KeyCode::KeyP,
            unfreeze_key: KeyCode::KeyO,
        }
    }
}
