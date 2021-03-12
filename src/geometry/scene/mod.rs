use cgmath::{Matrix4, SquareMatrix};

use std::collections::HashMap;

use super::palette::MeshID;

pub type InstanceID = u64;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Instance {
    pub mesh_id_: MeshID,
    pub transform_: Matrix4<f32>,
    pub dirty_: bool,
}

impl Instance {
    pub fn new(mesh_id: MeshID, transform: Option<Matrix4<f32>>) -> Instance {
        Instance {
            mesh_id_: mesh_id,
            transform_: match transform {
                Some(T) => T,
                None => Matrix4::identity(),
            },
            dirty_: true,
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Scene {
    instances_: HashMap<InstanceID, Instance>,
    curr_instance_id_: InstanceID,
    pub view_: Matrix4<f32>,
    pub projection_: Matrix4<f32>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            instances_: HashMap::new(),
            curr_instance_id_: 0,
            view_: Matrix4::identity(),
            projection_: Matrix4::identity(),
        }
    }

    pub fn create_instance(
        &mut self,
        mesh_id: MeshID,
        transform: Option<Matrix4<f32>>,
    ) -> InstanceID {
        self.instances_
            .insert(self.curr_instance_id_, Instance::new(mesh_id, transform));
        let instance_id = self.curr_instance_id_;
        self.curr_instance_id_ += 1;
        instance_id
    }

    pub fn get_mut_instance(&mut self, instance_id: &InstanceID) -> &mut Instance {
        self.instances_
            .get_mut(instance_id)
            .expect("No such instance")
    }

    pub fn get_instances(&self) -> Vec<Instance> {
        self.instances_.values().cloned().collect()
    }
}
