use std::ptr;

use ash::vk::{
    GraphicsPipelineCreateInfo, PipelineCache, PipelineCreateFlags, PipelineLayout, StructureType,
};
use ash::Device;

use crate::render_sequence::render_pass::pipeline::PipelineDescription;
use ash::version::DeviceV1_0;

pub mod conversion;
pub mod descriptor_set_layout;
mod input_descriptions;
mod pipeline_layout;
mod push_constant;
mod shader;

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Pipeline {
    pub pipeline_handle_: ash::vk::Pipeline,
    pub layout_: PipelineLayout,
}

impl Pipeline {
    /*
    pub fn from_pipeline_descriptor(
        device: &Device,
        pipeline_description: &PipelineDescription,
    ) -> Pipeline {
        let input_binding_descriptions = [input_descriptions::input_binding_from_description(
            &pipeline_description.input_,
        )]
        .to_vec();
        let attribute_descriptions = input_descriptions::input_attributes_from_descriptions(
            &pipeline_description.input_.attributes_,
        );

        let vertex_input_state = Pipeline::build_pipeline_vertex_input_state_create_info(
            &input_binding_descriptions,
            &attribute_descriptions,
        );

        let input_assembly_state = Pipeline::build_pipeline_input_assembly_state_create_info();

        let viewport = ash::vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 1920f32,
            height: 1080f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = ash::vk::Rect2D {
            offset: ash::vk::Offset2D { x: 0, y: 0 },
            extent: ash::vk::Extent2D {
                width: 1920,
                height: 1080,
            },
        };

        let viewport_state =
            Pipeline::build_pipeline_viewport_state_create_info(&viewport, &scissor);
        let rasterization_state = Pipeline::build_pipeline_rasterization_state_create_info();
        let multisample_state = Pipeline::build_pipeline_multisample_state_create_info();
        let depth_stencil_state = Pipeline::build_pipeline_depth_stencil_state_create_info();
        let color_blend_attachment_state = Pipeline::build_pipeline_color_blend_attachment_state();
        let color_blend_state =
            Pipeline::build_pipeline_color_blend_state_create_info(&color_blend_attachment_state);

        let mut vulkan_shaders = vec![];

        for shader in pipeline_description.shaders_.iter() {
            vulkan_shaders.push(shader::VulkanShader::new(device, shader));
        }

        let mut shader_stages = vec![];

        for shader in vulkan_shaders.iter() {
            shader_stages.push(shader.build_pipeline_shader_stage_create_info());
        }

        let mut push_constants = vec![];

        for push_constant_description in pipeline_description.push_constants_.iter() {
            push_constants.push(push_constant::from_description(push_constant_description));
        }

        let mut descriptor_set_layouts = vec![];

        for descriptor_set_description in pipeline_description.descriptor_sets_.iter() {
            descriptor_set_layouts.push(descriptor_set_layout::from_description(
                device,
                descriptor_set_description,
            ));
        }

        let pipeline_layout =
            pipeline_layout::new(device, &push_constants, &descriptor_set_layouts);

        let pipeline_create_info = GraphicsPipelineCreateInfo {
            s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ptr::null(),
            flags: PipelineCreateFlags::empty(),
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_state,
            p_input_assembly_state: &input_assembly_state,
            p_tessellation_state: ptr::null(),
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterization_state,
            p_multisample_state: &multisample_state,
            p_depth_stencil_state: &depth_stencil_state,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: ptr::null(),
            layout: pipeline_layout,
            render_pass: Default::default(),
            subpass: 0,
            base_pipeline_handle: Default::default(),
            base_pipeline_index: 0,
        };

        let pipeline = unsafe {
            device
                .create_graphics_pipelines(PipelineCache::null(), &[pipeline_create_info], None)
                .expect("Could not create pipeline")
        };

        Pipeline {
            pipeline_handle_: *pipeline.first().unwrap(),
            layout_: pipeline_layout,
        }
    }
*/
    /*
    pub fn from_reflection(device: &Device, shaders: &Vec<Shader>)
    {
        let mut reflections = vec![];

        for shader in shaders.iter()
        {
            reflections.push(shader.reflect());
        }
        let input_variables = reflections.first().expect("No reflections given").enumerate_input_variables(None).expect("Could not enumerate input variables");

        let vertex_input_binding_description = [input_descriptions::binding_description_from_reflection(&input_variables)].to_vec();
        let vertex_attribute_descriptions = input_descriptions::attribute_descriptions_from_reflection(&input_variables);

        let pipeline_vertex_input_state_create_info = Pipeline::build_pipeline_vertex_input_state_create_info(&vertex_input_binding_description, &vertex_attribute_descriptions);
        let pipeline_vertex_input_assembly_create_info = Pipeline::build_pipeline_input_assembly_state_create_info();

        let viewport = ash::vk::Viewport{
            x: 0.0,
            y: 0.0,
            width: 1920f32,
            height: 1080f32,
            min_depth: 0.0,
            max_depth: 1.0
        };

        let scissor = ash::vk::Rect2D{ offset: ash::vk::Offset2D { x: 0, y: 0 }, extent: ash::vk::Extent2D{ width: 1920, height: 1080 }};

        let pipeline_viewport_state_create_info = Pipeline::build_pipeline_viewport_state_create_info(&viewport, &scissor);
        let pipeline_rasterization_state_create_info = Pipeline::build_pipeline_rasterization_state_create_info();
        let pipeline_multisample_state_create_info = Pipeline::build_pipeline_multisample_state_create_info();
        let pipeline_depth_stencil_state_create_info = Pipeline::build_pipeline_depth_stencil_state_create_info();
        let color_blend_attachment_state = Pipeline::build_pipeline_color_blend_attachment_state();
        let pipeline_color_blend_state_create_info = Pipeline::build_pipeline_color_blend_state_create_info(&color_blend_attachment_state);

        let mut vulkan_shaders = vec![];

        for shader in shaders.iter()
        {
            vulkan_shaders.push(shader::VulkanShader::new(device, shader));
        }

        let mut pipeline_shader_stages = vec![];

        for vulkan_shader in vulkan_shaders.iter()
        {
            pipeline_shader_stages.push(vulkan_shader.build_pipeline_shader_stage_create_info());
        }


    }
    */

    /*
        pub fn new_forward_graphics_pipeline(device: &ash::Device, descriptor_set_layout: &ash::vk::DescriptorSetLayout, render_pass1: ash::vk::RenderPass, vert_path: &str, frag_path: &str) -> Pipeline
        {
            let vertex_input_binding_descriptions = [input_descriptions::get_vertex_input_binding_description()].to_vec();
            let vertex_attribute_descriptions = input_descriptions::get_vertex_attribute_descriptions().to_vec();

            let pipeline_vertex_input_state_create_info = Pipeline::build_pipeline_vertex_input_state_create_info(&vertex_input_binding_descriptions, &vertex_attribute_descriptions);
            let pipeline_vertex_input_assembly_create_info = Pipeline::build_pipeline_input_assembly_state_create_info();

            let viewport = ash::vk::Viewport{
                x: 0.0,
                y: 0.0,
                width: 1920f32,
                height: 1080f32,
                min_depth: 0.0,
                max_depth: 1.0
            };

            let scissor = ash::vk::Rect2D{ offset: ash::vk::Offset2D { x: 0, y: 0 }, extent: ash::vk::Extent2D{ width: 1920, height: 1080 }};

            let pipeline_viewport_state_create_info = Pipeline::build_pipeline_viewport_state_create_info(&viewport, &scissor);
            let pipeline_rasterization_state_create_info = Pipeline::build_pipeline_rasterization_state_create_info();
            let pipeline_multisample_state_create_info = Pipeline::build_pipeline_multisample_state_create_info();
            let pipeline_depth_stencil_state_create_info = Pipeline::build_pipeline_depth_stencil_state_create_info();
            let color_blend_attachment_state = Pipeline::build_pipeline_color_blend_attachment_state();
            let pipeline_color_blend_state_create_info = Pipeline::build_pipeline_color_blend_state_create_info(&color_blend_attachment_state);

            let vertex_shader = unsafe { shader::Shader::from_glsl(device, vert_path, ash::vk::ShaderStageFlags::VERTEX) };
            let fragment_shader = unsafe { shader::Shader::from_glsl(device, frag_path, ash::vk::ShaderStageFlags::FRAGMENT) };

            let shader_stages = [
                vertex_shader.build_pipeline_shader_stage_create_info(),
                fragment_shader.build_pipeline_shader_stage_create_info(),
            ];

            let pipeline_layout = unsafe { pipeline_layout::PipelineLayout::new(device, descriptor_set_layout) };

            let pipeline_create_info = [ash::vk::GraphicsPipelineCreateInfo{
                s_type: ash::vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: ptr::null(),
                flags: ash::vk::PipelineCreateFlags::empty(),
                stage_count: shader_stages.len() as u32,
                p_stages: shader_stages.as_ptr(),
                p_vertex_input_state: &pipeline_vertex_input_state_create_info,
                p_input_assembly_state: &pipeline_vertex_input_assembly_create_info,
                p_tessellation_state: ptr::null(),
                p_viewport_state: &pipeline_viewport_state_create_info,
                p_rasterization_state: &pipeline_rasterization_state_create_info,
                p_multisample_state: &pipeline_multisample_state_create_info,
                p_depth_stencil_state: &pipeline_depth_stencil_state_create_info,
                p_color_blend_state: &pipeline_color_blend_state_create_info,
                p_dynamic_state: ptr::null(),
                layout: pipeline_layout.layout_handle_,
                render_pass1: render_pass1,
                subpass: 0,
                base_pipeline_handle: ash::vk::Pipeline::null(),
                base_pipeline_index: -1
            }];

            let pipeline = unsafe { device.create_graphics_pipelines(ash::vk::PipelineCache::null(), &pipeline_create_info, None) }.expect("Could not create graphics pipeline");

            Pipeline{ pipeline_handle_: *pipeline.first().unwrap(), layout_: pipeline_layout }

        }
    */
    fn build_pipeline_vertex_input_state_create_info(
        vertex_input_binding_descriptions: &Vec<ash::vk::VertexInputBindingDescription>,
        vertex_input_attribute_descriptions: &Vec<ash::vk::VertexInputAttributeDescription>,
    ) -> ash::vk::PipelineVertexInputStateCreateInfo {
        ash::vk::PipelineVertexInputStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineVertexInputStateCreateFlags::empty(),
            vertex_binding_description_count: vertex_input_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_input_binding_descriptions.as_ptr(),
            vertex_attribute_description_count: vertex_input_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_input_attribute_descriptions.as_ptr(),
        }
    }
    /*
        fn build_pipeline_vertex_input_state_create_info_2d_hard() -> ash::vk::PipelineVertexInputStateCreateInfo
        {
            ash::vk::PipelineVertexInputStateCreateInfo{
                s_type: ash::vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                p_next: ptr::null(),
                flags: ash::vk::PipelineVertexInputStateCreateFlags::empty(),
                vertex_binding_description_count: 0,
                p_vertex_binding_descriptions: ptr::null(),
                vertex_attribute_description_count: 0,
                p_vertex_attribute_descriptions: ptr::null(),
            }
        }
    */
    fn build_pipeline_input_assembly_state_create_info(
    ) -> ash::vk::PipelineInputAssemblyStateCreateInfo {
        ash::vk::PipelineInputAssemblyStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology: ash::vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: ash::vk::FALSE,
        }
    }

    fn build_pipeline_viewport_state_create_info(
        viewport: &ash::vk::Viewport,
        scissor: &ash::vk::Rect2D,
    ) -> ash::vk::PipelineViewportStateCreateInfo {
        ash::vk::PipelineViewportStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: 1,
            p_viewports: viewport,
            scissor_count: 1,
            p_scissors: scissor,
        }
    }

    fn build_pipeline_rasterization_state_create_info(
    ) -> ash::vk::PipelineRasterizationStateCreateInfo {
        ash::vk::PipelineRasterizationStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineRasterizationStateCreateFlags::empty(),
            depth_clamp_enable: ash::vk::FALSE,
            rasterizer_discard_enable: ash::vk::FALSE,
            polygon_mode: ash::vk::PolygonMode::FILL,
            cull_mode: ash::vk::CullModeFlags::BACK,
            front_face: ash::vk::FrontFace::CLOCKWISE,
            depth_bias_enable: ash::vk::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        }
    }

    fn build_pipeline_multisample_state_create_info() -> ash::vk::PipelineMultisampleStateCreateInfo
    {
        ash::vk::PipelineMultisampleStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineMultisampleStateCreateFlags::empty(),
            rasterization_samples: ash::vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: ash::vk::FALSE,
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_coverage_enable: ash::vk::FALSE,
            alpha_to_one_enable: ash::vk::FALSE,
        }
    }

    fn build_pipeline_color_blend_attachment_state() -> ash::vk::PipelineColorBlendAttachmentState {
        ash::vk::PipelineColorBlendAttachmentState {
            blend_enable: ash::vk::FALSE,
            src_color_blend_factor: ash::vk::BlendFactor::ZERO,
            dst_color_blend_factor: ash::vk::BlendFactor::ZERO,
            color_blend_op: ash::vk::BlendOp::ADD,
            src_alpha_blend_factor: ash::vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: ash::vk::BlendFactor::ZERO,
            alpha_blend_op: ash::vk::BlendOp::ADD,
            color_write_mask: ash::vk::ColorComponentFlags::all(),
        }
    }

    fn build_pipeline_color_blend_state_create_info(
        color_blend_attachment_state: &ash::vk::PipelineColorBlendAttachmentState,
    ) -> ash::vk::PipelineColorBlendStateCreateInfo {
        ash::vk::PipelineColorBlendStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineColorBlendStateCreateFlags::empty(),
            logic_op_enable: ash::vk::FALSE,
            logic_op: ash::vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: color_blend_attachment_state,
            blend_constants: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn build_stencil_op_state() -> ash::vk::StencilOpState {
        ash::vk::StencilOpState {
            fail_op: ash::vk::StencilOp::KEEP,
            pass_op: ash::vk::StencilOp::KEEP,
            depth_fail_op: ash::vk::StencilOp::KEEP,
            compare_op: ash::vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        }
    }

    fn build_pipeline_depth_stencil_state_create_info(
    ) -> ash::vk::PipelineDepthStencilStateCreateInfo {
        let stencil_op_state = Pipeline::build_stencil_op_state();

        ash::vk::PipelineDepthStencilStateCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineDepthStencilStateCreateFlags::empty(),
            depth_test_enable: ash::vk::TRUE,
            depth_write_enable: ash::vk::TRUE,
            depth_compare_op: ash::vk::CompareOp::LESS,
            depth_bounds_test_enable: ash::vk::FALSE,
            stencil_test_enable: ash::vk::FALSE,
            front: stencil_op_state,
            back: stencil_op_state,
            min_depth_bounds: 0.0,
            max_depth_bounds: 1.0,
        }
    }

    /*
        pub unsafe fn new_2D_graphics_pipeline(device: &ash::Device, descriptor_set_layout: &ash::vk::DescriptorSetLayout, render_pass1: ash::vk::RenderPass) -> Pipeline
        {
            let vertex_input_binding_description = [input_descriptions::get_vertex_input_binding_description()].to_vec();
            let vertex_attribute_descriptions = input_descriptions::get_vertex_attribute_descriptions().to_vec();

           // let pipeline_vertex_input_state_create_info = Pipeline::build_pipeline_vertex_input_state_create_info(&vertex_input_binding_description, &vertex_attribute_descriptions);
            let pipeline_vertex_input_state_create_info = Pipeline::build_pipeline_vertex_input_state_create_info_2d_hard();
            let pipeline_vertex_input_assembly_create_info = Pipeline::build_pipeline_input_assembly_state_create_info();

            let viewport = ash::vk::Viewport{
                x: 0.0,
                y: 0.0,
                width: 1920f32,
                height: 1080f32,
                min_depth: 0.0,
                max_depth: 1.0
            };

            let scissor = ash::vk::Rect2D{ offset: ash::vk::Offset2D { x: 0, y: 0 }, extent: ash::vk::Extent2D{ width: 1920, height: 1080 }};

            let pipeline_viewport_state_create_info = Pipeline::build_pipeline_viewport_state_create_info(&viewport, &scissor);
            let pipeline_rasterization_state_create_info = Pipeline::build_pipeline_rasterization_state_create_info();
            let pipeline_multisample_state_create_info = Pipeline::build_pipeline_multisample_state_create_info();
            let pipeline_depth_stencil_state_create_info = Pipeline::build_pipeline_depth_stencil_state_create_info();

            let color_blend_attachment_state = Pipeline::build_pipeline_color_blend_attachment_state();

            let pipeline_color_blend_state_create_info = Pipeline::build_pipeline_color_blend_state_create_info(&color_blend_attachment_state);

            let vertex_shader = shader::Shader::from_glsl(device, "Engine/src/shaders/triangle_hard.vert", ash::vk::ShaderStageFlags::VERTEX);
            let fragment_shader = shader::Shader::from_glsl(device, "Engine/src/shaders/triangle_hard.frag", ash::vk::ShaderStageFlags::FRAGMENT);

            let shader_stages = [
            vertex_shader.build_pipeline_shader_stage_create_info(),
            fragment_shader.build_pipeline_shader_stage_create_info(),
            ];

            let pipeline_layout = pipeline_layout::PipelineLayout::new(device, descriptor_set_layout);

            let pipeline_create_info = [ash::vk::GraphicsPipelineCreateInfo{
                s_type: ash::vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: ptr::null(),
                flags: ash::vk::PipelineCreateFlags::empty(),
                stage_count: shader_stages.len() as u32,
                p_stages: shader_stages.as_ptr(),
                p_vertex_input_state: &pipeline_vertex_input_state_create_info,
                p_input_assembly_state: &pipeline_vertex_input_assembly_create_info,
                p_tessellation_state: ptr::null(),
                p_viewport_state: &pipeline_viewport_state_create_info,
                p_rasterization_state: &pipeline_rasterization_state_create_info,
                p_multisample_state: &pipeline_multisample_state_create_info,
                p_depth_stencil_state: &pipeline_depth_stencil_state_create_info,
                p_color_blend_state: &pipeline_color_blend_state_create_info,
                p_dynamic_state: ptr::null(),
                layout: pipeline_layout.layout_handle_,
                render_pass1: render_pass1,
                subpass: 0,
                base_pipeline_handle: ash::vk::Pipeline::null(),
                base_pipeline_index: -1
            }];

            let pipeline = device.create_graphics_pipelines(ash::vk::PipelineCache::null(), &pipeline_create_info, None).expect("Could not create graphics pipeline");

            Pipeline{ pipeline_handle_: *pipeline.first().unwrap(), layout_: pipeline_layout }
        }
    */
    /*
       pub fn shader_reflection_test(shader: &shader::Shader)
       {
           let shader_module = shader.reflect();

           let input_variables = shader_module.enumerate_input_variables(None).unwrap();
           let output_variables = shader_module.enumerate_output_variables(None).unwrap();
           let descriptor_sets = shader_module.enumerate_descriptor_sets(None).unwrap();
           let bindings = shader_module.enumerate_descriptor_bindings(None);
           let push_constants = shader_module.enumerate_push_constant_blocks(None);

       }
    */
}
