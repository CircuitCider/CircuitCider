//! code for making arrows
//!
//!

use bevy::{
    math::primitives::Cylinder,
    reflect::{Reflect, TypePath},
    render::mesh::Mesh,
};

#[derive(Default)]
pub enum Direction {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

//3d arrow mesh
#[derive(Default)]
pub struct Arrow3D {
    pub direction: Direction,
    pub shaft_length: f32,
    pub shaft_radius: f32,
}

impl From<Arrow3D> for Mesh {
    fn from(value: Arrow3D) -> Self {
        let cylinder = Cylinder::new(value.shaft_radius, value.shaft_length);
        let cylinder_info = Mesh::from(Cylinder::default());

        let topol = cylinder_info.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();

        //println!("topology info is {:#?}", topol);

        cylinder_info
    }
}
