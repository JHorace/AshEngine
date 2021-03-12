use ash::vk::PushConstantRange;

use super::conversion;
use crate::render_sequence::render_pass::pipeline::shader::push_constant::PushConstantDescription;

/*
pub fn from_description(description: &PushConstantDescription) -> PushConstantRange {
    PushConstantRange {
        stage_flags: conversion::shader_stage_to_vulkan_shader_stage(&description.shader_stage_),
        offset: description.offset_,
        size: description.size_,
    }
}
*/
/*
pub fn from_reflection(reflect_block_variables: &Vec<ReflectBlockVariable>, shader_stage: ShaderKind) -> Vec<PushConstantRange>
{
    let mut push_constant_ranges = vec![];


    for variable in reflect_block_variables.iter()
    {
        /// TODO: Determine which size and offset values should be used
        push_constant_ranges.push(    PushConstantRange{
            stage_flags: match shader_stage{
                ShaderKind::Vertex => ShaderStageFlags::VERTEX,
                ShaderKind::Fragment => ShaderStageFlags::FRAGMENT,
                ShaderKind::Compute => ShaderStageFlags::COMPUTE,
                ShaderKind::Geometry => ShaderStageFlags::GEOMETRY,
                ShaderKind::TessControl => ShaderStageFlags::TESSELLATION_CONTROL,
                ShaderKind::TessEvaluation => ShaderStageFlags::TESSELLATION_EVALUATION,
                _ => ShaderStageFlags::default(),
            },
            offset: variable.absolute_offset,
            size: variable.size,
        })
    }

    push_constant_ranges
}
 */
