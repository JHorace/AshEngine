use std::ptr;
use ash::version::DeviceV1_0;

use cgmath::Matrix4;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct PipelineLayout
{
   pub layout_handle_: ash::vk::PipelineLayout,
}

impl PipelineLayout{
    pub unsafe fn new(device: &ash::Device, descriptor_set_layout: &ash::vk::DescriptorSetLayout) -> PipelineLayout
    {
        let push_constant_range = ash::vk::PushConstantRange{
            stage_flags: ash::vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: std::mem::size_of::<Matrix4<u32>>() as u32 * 2,
        };

        let layout_create_info = ash::vk::PipelineLayoutCreateInfo{
            s_type: ash::vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 1,
            p_set_layouts: descriptor_set_layout,
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range
        };

        let layout_handle = device.create_pipeline_layout(&layout_create_info, None).expect("Could not create pipeline layout");

        PipelineLayout{ layout_handle_: layout_handle }
    }
}