pub mod shader;

use shader::{Shader, ShaderDescription};

pub struct PipelineDescription {
    pub shaders_: Vec<ShaderDescription>,
}

impl PipelineDescription {

    pub fn from_shaders(shaders: &Vec<Shader>) -> PipelineDescription
    {
        let mut shader_descriptions = vec![];

        for shader in shaders.iter()
        {
            shader_descriptions.push(ShaderDescription::from_shader(shader));
        }

        PipelineDescription{shaders_: shader_descriptions}
    }
    /*
    pub fn from_shaders(shaders: &Vec<Shader>) -> PipelineDescription
    {
        let mut reflections = vec![];

        for shader in shaders.iter()
        {
            reflections.push(shader.reflect());
        }

        let input = InputDescription::from_reflection(
            &reflections.first().unwrap().enumerate_input_variables(None).expect("Could not enumerate input variables"));

        let output = OutputDescription::from_reflection(
            &reflections.last().unwrap().enumerate_output_variables(None).expect("Could not enumerate output variables"));

        let mut push_constants : Vec<PushConstantDescription> = vec![];

        let mut descriptor_sets = vec![];

        for (i, reflection) in reflections.iter().enumerate()
        {
            let mut shader_push_constants = PushConstantDescription::from_reflections(
                &reflection.enumerate_push_constant_blocks(None).expect("Could not enumerate push constants"),
            &shaders[i].stage_);
            push_constants.append(& mut shader_push_constants);

            let mut shader_descriptor_sets = DescriptorSetDescription::from_reflections(
                &reflection.enumerate_descriptor_sets(None).expect("Could not enumerate descriptor sets"),
                &shaders[i].stage_);

            descriptor_sets.append(& mut shader_descriptor_sets);
        }

        PipelineDescription {
            shaders_: shaders.clone(),
            input_: input,
            output_: output,
            push_constants_: push_constants,
            descriptor_sets_: descriptor_sets,
        }
    }
    */
}
