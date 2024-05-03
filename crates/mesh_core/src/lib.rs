use bevy::render::mesh::VertexAttributeValues;


pub mod cone;
pub mod arrow;
pub mod ui;

// pub struct MeshAttributes {
//     pub positions: Vec<[f32; 3]>,
//     pub normals: Vec<[f32; 3]>,
//     pub uvs: Vec<[f32; 3]>,
// }

// pub enum MeshAttr<'a> {
//     POSITION(&'a mut Vec<[f32; 3]>),
//     //INDEX,
// }

pub enum MeshAttr {
    POSITION
}
/// returns a vec with mutable references to the values of mesh attributes.
/// If the attribute doesn't exist, returns none.
pub fn attr_to_vec(attr_fetch: Option<&mut VertexAttributeValues>) -> Option<Vec<Vec<&mut f32>>> {
    let attr = match attr_fetch {
        Some(attr) => attr,
        None => return None
    };

    match attr {
        VertexAttributeValues::Float32x3(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();
            
            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            // for mut n in vec {//.flat_map(|n| n) {
            //     return_vec.push(n);
            // }
            Some(return_vec)
        
        },
        VertexAttributeValues::Float32x2(vec) => {
            let mut return_vec: Vec<Vec<&mut f32>> = Vec::new();
            
            for i in vec.iter_mut() {
                let x = i.iter_mut().collect::<Vec<_>>();
                return_vec.push(x)
            }
            // for mut n in vec {//.flat_map(|n| n) {
            //     return_vec.push(n);
            // }
            Some(return_vec)
        
        },
        err => panic!("attribute retrieval not implemented for {:#?}", err)
    }
}

// pub fn mesh_attr_mut<'a, T>(mesh: &mut Mesh, mesh_attr: MeshAttr) -> T {
//     match T {

//     }
// }


// pub fn mesh_attr_mut<'a>(
//     mesh: &'a mut Mesh,
//     mesh_attr: MeshAttr
// ) -> Option<MeshAttr<'a>> {
//     match mesh_attr {
//         MeshAttr::POSITION(_,) => {
//             let Some(attr) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {return None};
            
//             match attr {
//                 bevy::render::mesh::VertexAttributeValues::Float32x3(vec) => {
//                     return Some(MeshAttr::POSITION(vec))
//                 },
//                 err => panic!("did not expect {:#?} as an attribute variant!", err)
//             }
//         },
//         //MeshAttr::INDEX(vec) => {return MeshAttr::INDEX(vec)}
//     }
// }