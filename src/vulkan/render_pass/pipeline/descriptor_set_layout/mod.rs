use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk::{
    DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags,
    DescriptorSetLayoutCreateInfo, StructureType,
};
use ash::Device;

use crate::render_sequence::render_pass::pipeline::shader::descriptor::{DescriptorSetDescription, DescriptorDescription};
use crate::render_sequence::render_pass::pipeline::shader::ShaderStage;

use super::conversion;
/*
pub fn from_description(
    device: &Device,
    description: &DescriptorSetDescription,
) -> DescriptorSetLayout {
    let mut bindings = vec![];

    for descriptor in description.descriptors_.iter() {
        bindings.push(binding_from_description(
            descriptor,
            &description.shader_stage_,
        ));
    }

    let descriptor_set_layout_create_info = DescriptorSetLayoutCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        p_next: ptr::null(),
        flags: DescriptorSetLayoutCreateFlags::empty(),
        binding_count: bindings.len() as u32,
        p_bindings: bindings.as_ptr(),
    };

    unsafe {
        device
            .create_descriptor_set_layout(&descriptor_set_layout_create_info, None)
            .expect("Could not create descriptor set layout")
    }
}

pub fn binding_from_description(
    description: &DescriptorDescription,
    shader_stage: &ShaderStage,
) -> DescriptorSetLayoutBinding {
    DescriptorSetLayoutBinding {
        binding: description.binding_,
        descriptor_type: conversion::descriptor_type_to_vulkan_descriptor_type(
            &description.descriptor_type_,
        ),
        descriptor_count: description.count_,
        stage_flags: conversion::shader_stage_to_vulkan_shader_stage(&shader_stage),
        p_immutable_samplers: ptr::null(),
    }
}
*/
/*
pub struct DescriptorSetLayout
{
    pub layout_handle_: ash::vk::DescriptorSetLayout,
}

impl DescriptorSetLayout
{
    pub unsafe fn new(device: &ash::Device, bindings: Vec<DescriptorSetLayoutBinding>) -> DescriptorSetLayout
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
*/

/*
pub fn from_reflection(device: &ash::Device, reflect_descriptor_set: &ReflectDescriptorSet, shader_stage: ShaderKind) -> DescriptorSetLayout
{
    let mut bindings = vec![];

    for binding in reflect_descriptor_set.bindings.iter()
    {
        bindings.push(convert_reflect_descriptor_binding(binding, shader_stage));
    }

    let layout_create_info = ash::vk::DescriptorSetLayoutCreateInfo{
        s_type: ash::vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        p_next: ptr::null(),
        flags: ash::vk::DescriptorSetLayoutCreateFlags::empty(),
        binding_count: bindings.len() as u32,
        p_bindings: bindings.as_ptr()
    };

    unsafe{device.create_descriptor_set_layout(&layout_create_info, None).expect("Could not create Descriptor Set Layout")}
}

fn convert_reflect_descriptor_binding(reflect_descriptor_binding: &ReflectDescriptorBinding, shader_stage: ShaderKind) -> DescriptorSetLayoutBinding
{
    DescriptorSetLayoutBinding{
        binding: reflect_descriptor_binding.binding,
        descriptor_type: match reflect_descriptor_binding.descriptor_type
        {
            ReflectDescriptorType::UniformBuffer => DescriptorType::UNIFORM_BUFFER,
            ReflectDescriptorType::UniformBufferDynamic => DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            ReflectDescriptorType::AccelerationStructureNV => DescriptorType::ACCELERATION_STRUCTURE_NV,
            ReflectDescriptorType::CombinedImageSampler => DescriptorType::COMBINED_IMAGE_SAMPLER,
            ReflectDescriptorType::InputAttachment => DescriptorType::INPUT_ATTACHMENT,
            ReflectDescriptorType::SampledImage => DescriptorType::SAMPLED_IMAGE,
            ReflectDescriptorType::Sampler => DescriptorType::SAMPLER,
            ReflectDescriptorType::StorageBuffer => DescriptorType::STORAGE_BUFFER,
            ReflectDescriptorType::StorageBufferDynamic => DescriptorType::STORAGE_BUFFER_DYNAMIC,
            ReflectDescriptorType::StorageImage => DescriptorType::STORAGE_IMAGE,
            ReflectDescriptorType::StorageTexelBuffer => DescriptorType::STORAGE_TEXEL_BUFFER,
            _ => DescriptorType::default()
        },

        descriptor_count: reflect_descriptor_binding.count,
        stage_flags: match shader_stage
        {
            ShaderKind::Vertex => ShaderStageFlags::VERTEX,
            ShaderKind::Fragment => ShaderStageFlags::FRAGMENT,
            ShaderKind::Compute => ShaderStageFlags::COMPUTE,
            ShaderKind::Geometry => ShaderStageFlags::GEOMETRY,
            ShaderKind::TessControl => ShaderStageFlags::TESSELLATION_CONTROL,
            ShaderKind::TessEvaluation => ShaderStageFlags::TESSELLATION_EVALUATION,
            _ => ShaderStageFlags::default(),
        },
        p_immutable_samplers: ptr::null(),
    }

}

pub fn build_descriptor_set_layout_bindings() -> [DescriptorSetLayoutBinding; 1]
{
    [
        ash::vk::DescriptorSetLayoutBinding{
            binding: 0,
            descriptor_type: DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
            stage_flags: ShaderStageFlags::VERTEX,
            p_immutable_samplers: ptr::null(),
        },
    ]
}
*/
