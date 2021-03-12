use std::fs;

use shaderc;
use shaderc::ShaderKind;
use spirv_reflect::ShaderModule;
use spirv_reflect::types::ReflectStorageClass::PushConstant;

use attribute::AttributeDescription;
use descriptor::DescriptorSetDescription;
use push_constant::PushConstantDescription;

pub mod attribute;
pub mod descriptor;
pub mod push_constant;

#[repr(C)]
#[derive(Clone, Debug)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
    Geometry,
    TesselationControl,
    TesselationEvaluation,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Shader {
    pub bytes_: Vec<u8>,
    pub stage_: ShaderStage,
}

pub struct ShaderDescription {
    pub shader_: Shader,
    pub input_: Vec<AttributeDescription>,
    pub output_: Vec<AttributeDescription>,
    pub push_constants_: Vec<PushConstantDescription>,
    pub descriptor_sets_: Vec<DescriptorSetDescription>,
}

impl ShaderDescription
{
    pub fn from_shader(shader: &Shader) -> ShaderDescription
    {
        let reflection = shader.reflect();

        let mut input = vec![];

        let reflect_input = reflection.enumerate_input_variables(None).expect("Could not enumerate input variables");

        for input_reflection in reflect_input.iter()
        {
            input.push(AttributeDescription::from_reflection(input_reflection));
        }

        let mut output = vec![];

        let reflect_output = reflection.enumerate_output_variables(None).expect("Could not enumerate output varibales");

        for output_reflection in reflect_output.iter()
        {
            output.push(AttributeDescription::from_reflection(output_reflection));
        }

        let mut push_constants = vec![];

        let reflect_push_constants = reflection.enumerate_push_constant_blocks(None).expect("Could not enumerate push constants");

        for push_constant_reflection in reflect_push_constants.iter()
        {
            push_constants.push(PushConstantDescription::from_reflection(push_constant_reflection));
        }

        let mut descriptor_sets = vec![];

        let reflect_descriptor_sets = reflection.enumerate_descriptor_sets(None).expect("Could not enumerate descriptor sets");

        for descriptor_set_reflection in reflect_descriptor_sets.iter()
        {
            descriptor_sets.push(DescriptorSetDescription::from_reflection(descriptor_set_reflection));
        }

        ShaderDescription{
            shader_: shader.clone(),
            input_: input,
            output_: output,
            push_constants_: push_constants,
            descriptor_sets_: descriptor_sets,
        }
    }
}

impl Shader {
    pub fn from_glsl(path: &str, shader_stage: ShaderStage) -> Shader {
        let mut compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();

        let glsl = fs::read_to_string(path).unwrap();
        let bytes = compiler
            .compile_into_spirv(
                glsl.as_str(),
                match shader_stage {
                    ShaderStage::Vertex => ShaderKind::Vertex,
                    ShaderStage::Fragment => ShaderKind::Fragment,
                    ShaderStage::Compute => ShaderKind::Compute,
                    ShaderStage::Geometry => ShaderKind::Geometry,
                    ShaderStage::TesselationControl => ShaderKind::TessControl,
                    ShaderStage::TesselationEvaluation => ShaderKind::TessEvaluation,
                },
                path,
                "main",
                Some(&options),
            )
            .unwrap()
            .as_binary_u8()
            .to_vec();

        Shader::from_bytes(bytes, shader_stage)
    }

    pub fn from_bytes(bytes: Vec<u8>, shader_stage: ShaderStage) -> Shader {
        Shader {
            bytes_: bytes,
            stage_: shader_stage,
        }
    }

    pub fn reflect(&self) -> ShaderModule {
        ShaderModule::load_u8_data(self.bytes_.as_slice()).expect("Could not reflect shader")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_shader_module_test() {
        let shader = super::Shader::from_glsl(
            "test/shaders/descriptor_set.vert",
            super::ShaderStage::Vertex,
        );

        let shader_module = shader.reflect();
    }

    #[test]
    fn enumerate_descriptor_set_test() {
        let shader = super::Shader::from_glsl(
            "test/shaders/descriptor_set.frag",
            super::ShaderStage::Fragment,
        );

        let shader_module = shader.reflect();

        let descriptor_bindings = shader_module.enumerate_descriptor_bindings(None).unwrap();
        let descriptor_sets = shader_module.enumerate_descriptor_sets(None).unwrap();
        let input_variables = shader_module.enumerate_input_variables(None).unwrap();
        let output_variables = shader_module.enumerate_output_variables(None).unwrap();
        let entry_points = shader_module.enumerate_entry_points().unwrap();
        let push_constants = shader_module.enumerate_push_constant_blocks(None).unwrap();
        let me = true;
    }
}
