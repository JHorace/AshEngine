use super::conversion;
use crate::render_sequence::render_pass::pipeline::shader::attribute::AttributeDescription;


use ash::vk::{VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate};

/*
pub fn input_binding_from_description(
    input_description: &AttributeDescription,
) -> VertexInputBindingDescription {
    VertexInputBindingDescription {
        binding: 0,
        stride: input_description.stride_,
        input_rate: VertexInputRate::VERTEX,
    }
}

pub fn input_attribute_from_description(
    attribute_description: &AttributeDescription,
) -> VertexInputAttributeDescription {
    VertexInputAttributeDescription {
        location: attribute_description.location_,
        binding: 0,
        format: conversion::format_to_vulkan_format(&attribute_description.format_),
        offset: attribute_description.offset_,
    }
}

pub fn input_attributes_from_descriptions(
    attribute_descriptions: &Vec<AttributeDescription>,
) -> Vec<VertexInputAttributeDescription> {
    let mut vulkan_attribute_descriptions = vec![];

    for attribute_description in attribute_descriptions.iter() {
        vulkan_attribute_descriptions.push(input_attribute_from_description(attribute_description));
    }

    vulkan_attribute_descriptions
}
*/
/*
pub fn binding_description_from_reflection(input_variables: &Vec<ReflectInterfaceVariable>) -> VertexInputBindingDescription
{
    let mut vertex_size = 0;

    for variable in input_variables.iter()
    {
        vertex_size += reflection_utils::size_of_reflect_numeric_trait(&variable.numeric);
    }

    VertexInputBindingDescription{
        binding: 0,
        stride: vertex_size,
        input_rate: VertexInputRate::VERTEX
    }
}

pub fn attribute_descriptions_from_reflection(input_variables: &Vec<ReflectInterfaceVariable>) -> Vec<VertexInputAttributeDescription>
{
    let mut attribute_descriptions = vec![];
    let mut curr_offset = 0u32;
    for variable in input_variables.iter()
    {
        attribute_descriptions.push(VertexInputAttributeDescription{
            location: variable.location,
            binding: 0,
            format: match variable.format
            {
                ReflectFormat::Undefined => Format::UNDEFINED,
                ReflectFormat::R32_UINT => Format::R32_UINT,
                ReflectFormat::R32_SINT => Format::R32_SINT,
                ReflectFormat::R32_SFLOAT => Format::R32_SFLOAT,
                ReflectFormat::R32G32_UINT => Format::R32G32_UINT,
                ReflectFormat::R32G32_SINT => Format::R32G32_SINT,
                ReflectFormat::R32G32_SFLOAT => Format::R32G32_SFLOAT,
                ReflectFormat::R32G32B32_UINT => Format::R32G32B32_UINT,
                ReflectFormat::R32G32B32_SINT => Format::R32G32B32_SINT,
                ReflectFormat::R32G32B32_SFLOAT => Format::R32G32B32_SFLOAT,
                ReflectFormat::R32G32B32A32_UINT => Format::R32G32B32A32_UINT,
                ReflectFormat::R32G32B32A32_SINT => Format::R32G32B32A32_SINT,
                ReflectFormat::R32G32B32A32_SFLOAT => Format::R32G32B32A32_SFLOAT,
            },
            offset: curr_offset,
        });

        curr_offset += reflection_utils::size_of_reflect_numeric_trait(&variable.numeric);
    }

    attribute_descriptions

}



pub fn get_vertex_input_binding_description() -> VertexInputBindingDescription
{
    ash::vk::VertexInputBindingDescription{
        binding: 0,
        stride: std::mem::size_of::<Vertex>() as u32,
        input_rate: ash::vk::VertexInputRate::VERTEX,
    }
}

pub fn get_vertex_attribute_descriptions() -> [VertexInputAttributeDescription; 4]
{
    [
        VertexInputAttributeDescription{
            location: 0,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, position_) as u32
        },
        VertexInputAttributeDescription{
            location: 1,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal_) as u32
        },
        VertexInputAttributeDescription{
            location: 2,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, color_) as u32
        },
        VertexInputAttributeDescription{
            location: 3,
            binding: 0,
            format: ash::vk::Format::R32G32_SFLOAT,
            offset:  offset_of!(Vertex, uv_) as u32
        },
    ]
}
*/
