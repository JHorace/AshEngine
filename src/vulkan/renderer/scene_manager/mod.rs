use cgmath::{Vector3, Vector4, Zero};
use super::buffer::Buffer;
use ash::vk::DescriptorBufferInfo;
use std::collections::HashMap;
use std::mem::size_of;
use crate::vulkan::physical_device::PhysicalDevice;
use std::os::raw::c_void;

#[repr(C)]
pub struct LightUBO
{
    pub position_: Vector4<f32>,
    pub color_: Vector4<f32>,
    /*
    pub constant_attenuation_: f32,
    pub linear_attenuation_: f32,
    pub quadratic_attenuation_: f32,
*/
}




pub struct SceneManager
{
    lights_: HashMap<u64, LightUBO>,
    uniform_buffers_: Vec<Buffer>,
}

impl SceneManager
{
    pub fn new(instance: &ash::Instance, device: &ash::Device, physical_device: &PhysicalDevice, max_lights: u64, num_frames: usize) -> SceneManager
    {
        let buffer_size = size_of::<LightUBO>() as u64 * max_lights;

        let mut buffers = vec![];

        for i in 0..num_frames
        {
            let mut buffer = Buffer::new(
                instance,
                device,
                physical_device,
                buffer_size,
                ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                ash::vk::MemoryPropertyFlags::HOST_VISIBLE | ash::vk::MemoryPropertyFlags::HOST_COHERENT
            );

            unsafe { buffer.map(device, 0, buffer_size) };

            buffers.push(buffer);
        }

        SceneManager{ lights_: HashMap::new(), uniform_buffers_: buffers}
    }

    pub fn create_light(&mut self) -> u64
    {
        let light_id = self.lights_.len();
        let light = LightUBO{
            position_: Vector4::zero(),
            color_: Vector4::zero(),
            /*
            constant_attenuation_: 0.0,
            linear_attenuation_: 0.0,
            quadratic_attenuation_: 0.0,
            */

        };
        self.lights_.insert(light_id as u64, light);
        light_id as u64
    }

    pub fn get_mut_light(& mut self, light_id: u64) -> & mut LightUBO
    {
        self.lights_.get_mut(&light_id).expect("No such light")
    }

    pub fn update(&mut self, curr_frame: u32)
    {
        for(key, light) in self.lights_.iter_mut()
        {
            println!("{}", light.color_.x);
            let data_ptr : * mut c_void = light as * mut _ as * mut c_void;
            unsafe{self.uniform_buffers_[curr_frame as usize].copy_from_data(data_ptr, size_of::<LightUBO>(), size_of::<LightUBO>() as u64 * key)};
        }
    }

    pub fn get_descriptor_buffer_info(&self) ->DescriptorBufferInfo
    {
        println!("{}", self.lights_.len());
        DescriptorBufferInfo{
            buffer: self.uniform_buffers_[0].buffer_handle_,
            offset: 0,
            range: size_of::<LightUBO>() as u64 * self.lights_.len() as u64,
        }
    }
}