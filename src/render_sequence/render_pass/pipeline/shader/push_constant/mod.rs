use spirv_reflect::types::ReflectBlockVariable;

pub struct PushConstantDescription {
    pub offset_: u32,
    pub size_: u32,
}

impl PushConstantDescription {
    pub fn from_reflection(reflect_block_variable: &ReflectBlockVariable) -> PushConstantDescription {
        PushConstantDescription {
            offset_: reflect_block_variable.absolute_offset,
            size_: reflect_block_variable.size,
        }
    }

    pub fn from_reflections(
        reflect_block_variables: &Vec<ReflectBlockVariable>) -> Vec<PushConstantDescription> {
        let mut push_constant_ranges = vec![];

        for variable in reflect_block_variables.iter() {
            push_constant_ranges.push(PushConstantDescription::from_reflection(
                variable));
        }

        push_constant_ranges
    }
}
