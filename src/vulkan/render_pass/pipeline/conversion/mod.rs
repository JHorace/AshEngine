use crate::render_sequence::reflection_utils::format::Format;
use crate::render_sequence::render_pass::pipeline::shader::ShaderStage;
use crate::render_sequence::render_pass::pipeline::shader::descriptor::DescriptorType;

use ash::vk;

pub fn format_to_vulkan_format(format: &Format) -> vk::Format {
    match format {
        Format::UNDEFINED => vk::Format::UNDEFINED,
        Format::R32_UINT => vk::Format::R32_UINT,
        Format::R32_SINT => vk::Format::R32_SINT,
        Format::R32_SFLOAT => vk::Format::R32_SFLOAT,
        Format::R32G32_UINT => vk::Format::R32G32_UINT,
        Format::R32G32_SINT => vk::Format::R32G32_SINT,
        Format::R32G32_SFLOAT => vk::Format::R32G32_SFLOAT,
        Format::R32G32B32_UINT => vk::Format::R32G32B32_UINT,
        Format::R32G32B32_SINT => vk::Format::R32G32B32_SINT,
        Format::R32G32B32_SFLOAT => vk::Format::R32G32B32_SFLOAT,
        Format::R32G32B32A32_UINT => vk::Format::R32G32B32A32_UINT,
        Format::R32G32B32A32_SINT => vk::Format::R32G32B32A32_SINT,
        Format::R32G32B32A32_SFLOAT => vk::Format::R32G32B32A32_SFLOAT,
    }
}

pub fn shader_stage_to_vulkan_shader_stage(shader_stage: &ShaderStage) -> vk::ShaderStageFlags {
    match shader_stage {
        ShaderStage::Vertex => vk::ShaderStageFlags::VERTEX,
        ShaderStage::Fragment => vk::ShaderStageFlags::FRAGMENT,
        ShaderStage::Compute => vk::ShaderStageFlags::COMPUTE,
        ShaderStage::Geometry => vk::ShaderStageFlags::GEOMETRY,
        ShaderStage::TesselationControl => vk::ShaderStageFlags::TESSELLATION_CONTROL,
        ShaderStage::TesselationEvaluation => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
    }
}

pub fn descriptor_type_to_vulkan_descriptor_type(
    descriptor_type: &DescriptorType,
) -> vk::DescriptorType {
    match descriptor_type {
        DescriptorType::Undefined => vk::DescriptorType::UNIFORM_BUFFER,
        DescriptorType::Sampler => vk::DescriptorType::SAMPLER,
        DescriptorType::CombinedImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        DescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
        DescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
        DescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
        DescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
        DescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
        DescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
        DescriptorType::UniformBufferDynamic => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
        DescriptorType::StorageBufferDynamic => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
        DescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
        DescriptorType::AccelerationStructureNV => vk::DescriptorType::ACCELERATION_STRUCTURE_NV,
    }
}
