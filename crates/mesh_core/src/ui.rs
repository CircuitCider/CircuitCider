use std::{collections::HashMap, fmt::Display, ops::DerefMut};

use bevy::{ecs::system::Resource, prelude::{Deref, DerefMut}, reflect::Reflect};
use bevy_inspector_egui::egui::{Color32, RichText, Ui};
use egui_extras::TableRow;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};



#[derive(Resource, Default, Deref, DerefMut)]
pub struct TablePick<T: Display>(HashMap<String, T>);

impl<T: IntoEnumIterator + Display + Eq + Copy + Default> TablePick<T> {
    /// adds table with options to switch between.
    pub fn get_table(&mut self, ui: &mut Ui) -> &mut Self{
        ui.horizontal(|ui| {
            
            for attr in T::iter() {
                let key = &(attr.to_string());
                let contains_key = (**self).contains_key(key);

                let color = if contains_key {
                    Color32::WHITE
                } else {
                    Color32::GRAY
                };
                if ui.button(
                    RichText::new( attr.to_string())
                    .color(color) 
                )
                .clicked() {
                    if contains_key {
                        (**self).remove(key);

                    } else {
                        (**self).insert(key.clone(), attr);
                    }
                }
            }
        });
        self
    } 
    /// layout headers for table
    pub fn layout_headers(&mut self, ui: &mut TableRow) {
            
        for attr in T::iter() {
            let key = &(attr.to_string());
            let contains_key = (**self).contains_key(key);

            let color = if contains_key {
                Color32::WHITE
            } else {
                Color32::GRAY
            };
            ui.col(|ui| {
                if ui.button(
                    RichText::new( attr.to_string())
                    .color(color) 
                )
                .clicked() {
                    if contains_key {
                        (**self).remove(key);
    
                    } else {
                        (**self).insert(key.clone(), attr);
                    }
                }
            });

        }
        //self
    }
    /// layout rows for collums of table
    pub fn layout_rows(&self, ui: &mut Ui) {

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