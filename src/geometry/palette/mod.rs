use std::collections::HashMap;

use super::mesh::Mesh;

pub type MeshID = u64;

pub struct Palette {
    pub meshes_: HashMap<MeshID, Mesh>,
    curr_mesh_id_: MeshID,
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            meshes_: HashMap::new(),
            curr_mesh_id_: 0,
        }
    }

    pub fn load_mesh(&mut self, mesh: &Mesh) -> MeshID {
        let mesh_id_ = self.curr_mesh_id_;

        self.meshes_.insert(mesh_id_, mesh.clone());

        self.curr_mesh_id_ += 1;

        mesh_id_
    }
}
