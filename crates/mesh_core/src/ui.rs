use std::{collections::HashMap, fmt::Display, ops::DerefMut};

use bevy::{
    ecs::system::Resource,
    prelude::{Deref, DerefMut},
    reflect::Reflect,
};
use bevy_inspector_egui::egui::{scroll_area::ScrollBarVisibility, Color32, RichText, Ui};
use egui_extras::{Column, Table, TableBody, TableBuilder, TableRow};
use strum::IntoEnumIterator;
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
