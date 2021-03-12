use ash::version::DeviceV1_0;
use ash::vk::{
    DescriptorSetLayout, PipelineLayout, PipelineLayoutCreateFlags, PipelineLayoutCreateInfo,
    PushConstantRange, StructureType,
};
use ash::Device;

use std::ptr;

pub fn new(
    device: &Device,
    push_constants: &Vec<PushConstantRange>,
    descriptor_set_layouts: &Vec<DescriptorSetLayout>,
) -> PipelineLayout {
    let pipeline_layout_create_info = PipelineLayoutCreateInfo {
        s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        p_next: ptr::null(),
        flags: PipelineLayoutCreateFlags::empty(),
        set_layout_count: descriptor_set_layouts.len() as u32,
        p_set_layouts: descriptor_set_layouts.as_ptr(),
        push_constant_range_count: push_constants.len() as u32,
        p_push_constant_ranges: push_constants.as_ptr(),
    };

    unsafe {
        device
            .create_pipeline_layout(&pipeline_layout_create_info, None)
            .expect("Could not create pipeline layout")
    }
}

/*
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct PipelineLayout
{
   pub layout_handle_: ash::vk::PipelineLayout,
}

impl PipelineLayout{
    pub unsafe fn new(device: &ash::Device, descriptor_set_layout: &ash::vk::DescriptorSetLayout) -> PipelineLayout
    {
        let push_constant_ranges = [ash::vk::PushConstantRange{
            stage_flags: ash::vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: std::mem::size_of::<Matrix4<u32>>() as u32 * 2,
        }];


        let layout_create_info = ash::vk::PipelineLayoutCreateInfo{
            s_type: ash::vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 1,
            p_set_layouts: descriptor_set_layout,
            push_constant_range_count: 1,
            p_push_constant_ranges: push_constant_ranges.as_ptr()
        };

        let layout_handle = device.create_pipeline_layout(&layout_create_info, None).expect("Could not create pipeline layout");

        PipelineLayout{ layout_handle_: layout_handle }
    }

}
*/
/*
pub fn from_reflection(device: &ash::Device, shaders: &Vec<Shader>)
{
    let mut reflections = vec![];

    for shader in shaders.iter()
    {
        reflections.push(shader.reflect());
    }

}
*/
