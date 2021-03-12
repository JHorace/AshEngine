use super::mesh;

use std::path::Path;

pub struct Model {
    pub meshes_: Vec<mesh::Mesh>,
}

impl Model {
    pub fn from_obj(path: &Path) -> Model {
        let mut meshes = vec![];

        let tobj_model = tobj::load_obj(path).expect("Could not load obj file");

        let (models, materials) = tobj_model;

        for model in models.iter() {
            let mesh = mesh::Mesh::from_tobj_mesh(&model.mesh);
            meshes.push(mesh);
        }

        Model { meshes_: meshes }
    }
}
