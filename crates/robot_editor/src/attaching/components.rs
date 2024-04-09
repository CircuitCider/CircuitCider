use bevy::prelude::*;

/// marker for objects that are not yet a part of a structure but could be
/// (placed build mode models)
#[derive(Component, Default)]
pub struct AttachCandidate;