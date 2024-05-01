use std::{fmt::Display, ops::DerefMut};

use bevy::{ecs::system::Resource, prelude::{Deref, DerefMut}, reflect::Reflect};
use bevy_inspector_egui::egui::{Color32, RichText, Ui};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};



#[derive(Resource, Default, Deref, DerefMut)]
pub struct TablePick<T>(T);

impl<T: IntoEnumIterator + Display + Eq + Copy> TablePick<T> {
    /// adds table with options to switch between.
    pub fn table(&mut self, ui: &mut Ui) -> T{
        ui.horizontal(|ui| {
            

            for attr in T::iter() {
                let color = if **self == attr {
                    Color32::WHITE
                } else {
                    Color32::GRAY
                };

                if ui.button(
                    RichText::new( attr.to_string())
                    .color(color) 
                )
                .clicked() {
                    **self = attr
                }
            }
            
        });
        **self
    } 
}

#[derive(Default, Clone, Copy, Reflect, Debug, PartialEq, Eq, EnumIter, Display)]
pub enum MeshAttributes {
    #[default]
    POSITION,
    INDICE,
    NORMAL,
    UV,
}

// pub struct TableSelection<T, U>([])