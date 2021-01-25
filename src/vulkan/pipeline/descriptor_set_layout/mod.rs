use ash::vk::DescriptorType;
use std::ptr;
use ash::version::DeviceV1_0;

pub struct DescriptorSetLayout
{
    pub layout_handle_: ash::vk::DescriptorSetLayout,
}

impl DescriptorSetLayout
{
    pub unsafe fn new(device: &ash::Device, bindings: Vec<ash::vk::DescriptorSetLayoutBinding>) -> DescriptorSetLayout
    {
        let layout_create_info = ash::vk::DescriptorSetLayoutCreateInfo{
            s_type: ash::vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr()
        };
        
        let layout_handle = device.create_descriptor_set_layout(&layout_create_info, None).expect("Could not create Descriptor Set Layout");
        
        DescriptorSetLayout{ layout_handle_: layout_handle }
    }
}

pub fn build_descriptor_set_layout_bindings() -> [ash::vk::DescriptorSetLayoutBinding; 3]
{
    [
        ash::vk::DescriptorSetLayoutBinding{
            binding: 0,
            descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: ash::vk::ShaderStageFlags::VERTEX,
            p_immutable_samplers: ptr::null(),
        },
        ash::vk::DescriptorSetLayoutBinding{
            binding: 1,
            descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: ash::vk::ShaderStageFlags::FRAGMENT,
            p_immutable_samplers: ptr::null(),
        },
        ash::vk::DescriptorSetLayoutBinding{
            binding: 2,
            descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: ash::vk::ShaderStageFlags::FRAGMENT,
            p_immutable_samplers: ptr::null(),
        }
    ]
}
