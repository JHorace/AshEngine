use spirv_reflect::types::ReflectFormat;

#[allow(non_camel_case_types)]
pub enum Format {
    UNDEFINED,
    R32_UINT,
    R32_SINT,
    R32_SFLOAT,
    R32G32_UINT,
    R32G32_SINT,
    R32G32_SFLOAT,
    R32G32B32_UINT,
    R32G32B32_SINT,
    R32G32B32_SFLOAT,
    R32G32B32A32_UINT,
    R32G32B32A32_SINT,
    R32G32B32A32_SFLOAT,
}

impl Format {
    pub fn from_reflect_format(reflect_format: ReflectFormat) -> Format {
        match reflect_format {
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
        }
    }
}
