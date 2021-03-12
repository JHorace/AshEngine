use super::super::geometry::scene::Instance;
use super::buffer::Buffer;
use super::physical_device::PhysicalDevice;
use ash;
use ash::vk::DescriptorBufferInfo;

use std::os::raw::c_void;

pub struct UniformManager {
    uniform_buffers_: Vec<Buffer>,
    ubo_size_: u64,
}

impl UniformManager {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &PhysicalDevice,
        max_instances: u64,
        num_frames: u32,
        ubo_size: u64,
    ) -> UniformManager {
        let buffer_size = ubo_size as u64 * max_instances;

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

        UniformManager {
            uniform_buffers_: buffers,
            ubo_size_: ubo_size,
        }
    }

    pub fn update_uniforms(
        &mut self,
        curr_frame: u32,
        instances: &Vec<Instance>,
    ) -> Vec<DescriptorBufferInfo> {
        let mut descriptor_buffer_infos = vec![];

        for (i, instance) in instances.iter().enumerate() {
            let data_ptr: *const c_void = &instance.transform_ as *const _ as *const c_void;
            unsafe {
                self.uniform_buffers_[curr_frame as usize].copy_from_data(
                    data_ptr,
                    self.ubo_size_,
                    self.ubo_size_ * i as u64,
                )
            };
            descriptor_buffer_infos.push(DescriptorBufferInfo {
                buffer: self.uniform_buffers_[curr_frame as usize].buffer_handle_,
                offset: self.ubo_size_ * i as u64,
                range: self.ubo_size_,
            });
        }

        descriptor_buffer_infos
    }
    /*
    pub fn get_descriptor_buffer_infos(& self, curr_frame: u32, num_instances: u32) -> Vec<ash::vk::DescriptorBufferInfo>
    {
        let mut descriptor_buffer_infos = vec![];
        for i in 0..num_instances
        {
            descriptor_buffer_infos.push(ash::vk::DescriptorBufferInfo{
                buffer: self.uniform_buffers_[curr_frame as usize].buffer_handle_,
                offset: self.ubo_size_ * i as u64,
                range: self.ubo_size_
            });
        }

        descriptor_buffer_infos
    }
    */
}
