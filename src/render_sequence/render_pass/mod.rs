pub mod pipeline;

use pipeline::PipelineDescription;

pub struct RenderPassDescription {
    pub mainpass_: PipelineDescription,
    pub subpasses_: Vec<PipelineDescription>,
}

impl RenderPassDescription {

    pub fn from_pipelines(mainpass: PipelineDescription, subpasses: Vec<PipelineDescription>) -> RenderPassDescription
    {
        RenderPassDescription{ mainpass_: mainpass, subpasses_: subpasses }
    }

}

#[cfg(test)]
mod tests {
    use crate::render_sequence::render_pass::pipeline::PipelineDescription;
    /*
        #[test]
        fn create_render_pass_description_test()
        {
            let vert = super::pipeline::shader::Shader::from_glsl("test/shaders/descriptor_set.vert", super::pipeline::shader::ShaderStage::Vertex);
            let frag = super::pipeline::shader::Shader::from_glsl("test/shaders/descriptor_set.frag", super::pipeline::shader::ShaderStage::Fragment);

            let shaders = vec![vert, frag];

            let pipeline = PipelineDescription::from_shaders(&shaders);

            let breakpoint = true;
        }
    */
}
