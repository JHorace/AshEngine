use super::buffer::Buffer;
use super::physical_device::PhysicalDevice;
use ash::vk::DescriptorBufferInfo;
use ash::vk::DescriptorSet;
use cgmath::{Matrix4, SquareMatrix};
use std::collections::HashMap;
use std::mem::size_of;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct UBO {
    pub model_: Matrix4<f32>,
    pub view_: Matrix4<f32>,
    pub projection_: Matrix4<f32>,
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Instance {
    pub mesh_id_: u64,
    ubo_: UBO,
    pub descriptor_set_: DescriptorSet,
    dirty_: bool,
}

impl Instance {
    pub fn new(mesh_id: u64, descriptor_set: DescriptorSet) -> Instance {
        Instance {
            mesh_id_: mesh_id,
            ubo_: UBO {
                model_: Matrix4::identity(),
                view_: Matrix4::identity(),
                projection_: Matrix4::identity(),
            },
            descriptor_set_: descriptor_set,
            dirty_: true,
        }
    }

    pub fn update(&mut self, ubo: UBO) {
        self.ubo_ = ubo;
        self.dirty_ = true;
    }
}

pub struct InstanceManager {
    instances_: HashMap<u64, (Instance, DescriptorBufferInfo)>,
    uniform_buffers_: Vec<Buffer>,
    curr_instance_id_: u64,
}

impl InstanceManager {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &PhysicalDevice,
        max_objects: u64,
        num_frames: usize,
    ) -> InstanceManager {
        let buffer_size = size_of::<UBO>() as u64 * max_objects;

        let mut buffers = vec![];

        for _ in 0..num_frames {
            let mut buffer = Buffer::new(
                instance,
                device,
                physical_device,
                buffer_size,
                ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                    | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
            );

            unsafe { buffer.map(device, 0, buffer_size) };

            buffers.push(buffer);
        }

        InstanceManager {
            instances_: HashMap::new(),
            uniform_buffers_: buffers,
            curr_instance_id_: 0,
        }
    }

    pub fn create_instance(&mut self, mesh_id: u64, descriptor_set: DescriptorSet) -> u64 {
        let instance_id = self.curr_instance_id_;
        let instance = Instance::new(mesh_id, descriptor_set);
        let descriptor_buffer_info = DescriptorBufferInfo {
            buffer: self.uniform_buffers_[0].buffer_handle_,
            offset: 0 * instance_id,
            range: size_of::<UBO>() as u64,
        };
        self.instances_
            .insert(instance_id, (instance, descriptor_buffer_info));
        self.curr_instance_id_ = instance_id + 1;
        instance_id
    }

    pub fn update_instance(&mut self, instance_id: &u64, ubo: UBO) {
        let (instance, ..) = self
            .instances_
            .get_mut(instance_id)
            .expect("No such instance");
        instance.update(ubo);
    }

    pub fn update(&mut self, curr_frame: u32) {
        for (key, (instance, ..)) in self.instances_.iter_mut() {
            if instance.dirty_ {
                let data_ptr: *mut c_void = &mut instance.ubo_ as *mut _ as *mut c_void;
                unsafe {
                    self.uniform_buffers_[curr_frame as usize].copy_from_data(
                        data_ptr,
                        size_of::<UBO>() as u64,
                        size_of::<UBO>() as u64 * key,
                    )
                }
            }
        }
    }

    pub fn get_instances(&self) -> Vec<(Instance, DescriptorBufferInfo)> {
        self.instances_.values().cloned().collect()
    }
}
