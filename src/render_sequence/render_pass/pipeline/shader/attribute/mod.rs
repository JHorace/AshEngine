use crate::render_sequence::reflection_utils::format::Format;

use spirv_reflect::types::ReflectInterfaceVariable;

pub struct AttributeDescription {
    pub location_: u32,
    pub offset_: u32,
    pub format_: Format,
    pub present_: bool,
}

impl AttributeDescription {
    pub fn from_reflections(
        attribute_reflections: &Vec<ReflectInterfaceVariable>,
    ) -> Vec<AttributeDescription> {
        let mut attribute_descriptors = vec![];
        let mut curr_offset = 0u32;
        for reflection in attribute_reflections.iter() {
            attribute_descriptors.push(AttributeDescription {
                location_: reflection.location,
                offset_: curr_offset,
                format_: Format::from_reflect_format(reflection.format),
                present_: false,
            })
        }

        attribute_descriptors
    }

    pub fn from_reflection(
        attribute_reflection: &ReflectInterfaceVariable,
    ) -> AttributeDescription {
        AttributeDescription {
            location_: attribute_reflection.location,
            offset_: 0,
            format_: Format::from_reflect_format(attribute_reflection.format),
            present_: false,
        }
    }
}
