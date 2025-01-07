use bevy::reflect::Reflect;
use strum_macros::{Display, EnumIter};

/// struct with methods for quickly creating tables with associated formatting.

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum MeshAttributes {
    #[default]
    POSITION,
    INDICE,
    NORMAL,
    UV,
}

// pub struct TableSelection<T, U>([])
