use bevy::{ecs::schedule::States, reflect::Reflect};



#[derive(Clone, Copy, Reflect, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum RobotEditorState {
    Active,
    #[default]
    Inactive,
}