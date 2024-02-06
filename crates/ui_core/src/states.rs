use bevy::prelude::*;

#[derive(Clone, Copy, Reflect, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum MainMenuState {
    #[default]
    Active,
    Inactive,
}
