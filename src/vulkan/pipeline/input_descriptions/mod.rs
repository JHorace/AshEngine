use super::super::super::geometry;

use memoffset::offset_of;

pub fn get_vertex_input_binding_description() -> ash::vk::VertexInputBindingDescription
{
    ash::vk::VertexInputBindingDescription{
        binding: 0,
        stride: std::mem::size_of::<geometry::vertex::Vertex>() as u32,
        input_rate: ash::vk::VertexInputRate::VERTEX,
    }
}

pub fn get_vertex_attribute_descriptions() -> [ash::vk::VertexInputAttributeDescription; 4]
{
    [
        ash::vk::VertexInputAttributeDescription{
            location: 0,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(geometry::vertex::Vertex, position_) as u32
        },
        ash::vk::VertexInputAttributeDescription{
            location: 1,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(geometry::vertex::Vertex, normal_) as u32
        },
        ash::vk::VertexInputAttributeDescription{
            location: 2,
            binding: 0,
            format: ash::vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(geometry::vertex::Vertex, color_) as u32
        },
        ash::vk::VertexInputAttributeDescription{
            location: 3,
            binding: 0,
            format: ash::vk::Format::R32G32_SFLOAT,
            offset:  offset_of!(geometry::vertex::Vertex, uv_) as u32
        },
    ]
}