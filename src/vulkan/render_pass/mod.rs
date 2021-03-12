pub mod pipeline;

use ash::version::DeviceV1_0;
use ash::vk;
use ash::vk::{
    AccessFlags, AttachmentDescription, AttachmentDescriptionFlags, AttachmentLoadOp,
    AttachmentReference, AttachmentStoreOp, DependencyFlags, ImageLayout, PipelineBindPoint,
    PipelineStageFlags, RenderPassCreateFlags, RenderPassCreateInfo, SampleCountFlags,
    StructureType, SubpassDependency, SubpassDescription, SubpassDescriptionFlags,
};
use ash::Device;
use std::ptr;

use pipeline::Pipeline;

use pipeline::conversion;

use crate::render_sequence::render_pass::RenderPassDescription;
use crate::render_sequence::render_pass::pipeline::shader::attribute::AttributeDescription;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RenderPass {
    pub vk_render_pass_: vk::RenderPass,
    pub mainpass_: Pipeline,
    pub subpasses_: Vec<Pipeline>,
}

impl RenderPass {

    /*
    pub fn from_description(
        device: &Device,
        description: &RenderPassDescription,
        surface_format: Option<vk::Format>,
        depth_format: Option<vk::Format>,
    ) -> RenderPass {
        let mut color_attachments = vec![];
        let mut color_attachment_refs = vec![];

        for attachment_description in description.pipeline_.output_.attributes_.iter() {
            color_attachments.push(RenderPass::attachment_from_description(
                attachment_description,
                surface_format,
            ));
            color_attachment_refs.push(AttachmentReference {
                attachment: attachment_description.location_,
                layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            });
        }

        let depth_attachment = RenderPass::build_depth_attachment(depth_format.unwrap());
        let depth_attachment_reference = AttachmentReference {
            attachment: color_attachments.len() as u32,
            layout: ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpass = SubpassDescription {
            flags: SubpassDescriptionFlags::empty(),
            pipeline_bind_point: PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: 1,
            p_color_attachments: color_attachment_refs.as_ptr(),
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: [depth_attachment_reference].as_ptr(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        };

        let mut attachments = color_attachments.clone();
        attachments.push(depth_attachment);

        let subpass_dependencies = SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: AccessFlags::empty(),
            dst_access_mask: AccessFlags::COLOR_ATTACHMENT_READ
                | AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: DependencyFlags::empty(),
        };

        let render_pass_create_info = RenderPassCreateInfo {
            s_type: StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 1,
            p_dependencies: &subpass_dependencies,
        };

        let render_pass = unsafe {
            device
                .create_render_pass(&render_pass_create_info, None)
                .expect("could not create render pass")
        };
        let pipeline = Pipeline::from_pipeline_descriptor(device, &description.pipeline_);

        RenderPass {
            vk_render_pass_: Default::default(),
            entry_: pipeline,
            subpass_pipelines_: vec![],
        }
    }

    fn attachment_from_description(
        description: &AttributeDescription,
        surface_format: Option<vk::Format>,
    ) -> AttachmentDescription {
        AttachmentDescription {
            flags: AttachmentDescriptionFlags::empty(),
            format: if description.present_ {
                surface_format.unwrap()
            } else {
                conversion::format_to_vulkan_format(&description.format_)
            },
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::STORE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: if description.present_ {
                ImageLayout::PRESENT_SRC_KHR
            } else {
                ImageLayout::COLOR_ATTACHMENT_OPTIMAL
            },
        }
    }

    fn build_color_attachment(surface_format: vk::Format) -> AttachmentDescription {
        AttachmentDescription {
            flags: AttachmentDescriptionFlags::empty(),
            format: surface_format,
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::STORE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: ImageLayout::PRESENT_SRC_KHR,
        }
    }

    fn build_depth_attachment(depth_format: ash::vk::Format) -> AttachmentDescription {
        AttachmentDescription {
            flags: AttachmentDescriptionFlags::empty(),
            format: depth_format,
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::DONT_CARE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        }
    }

     */
}

/*
#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct RenderPass{
    pub handle_: ash::vk::RenderPass,
}

impl RenderPass{
    pub fn build_forward_render_pass(device:& ash::Device, surface_format: ash::vk::Format, depth_format: ash::vk::Format) ->RenderPass
    {
        let color_attachment = RenderPass::build_color_attachment_description(surface_format);

        let color_attachment_reference = ash::vk::AttachmentReference{
            attachment: 0,
            layout: ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment = RenderPass::build_depth_attachment_description(depth_format);

        let depth_attachment_reference = ash::vk::AttachmentReference{
            attachment: 1,
            layout: ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpass = ash::vk::SubpassDescription{
            flags: ash::vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: ash::vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_reference,
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: &depth_attachment_reference,
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null()
        };

        let attachments = [color_attachment, depth_attachment];

        let subpass_dependencies = ash::vk::SubpassDependency{
            src_subpass: ash::vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: ash::vk::AccessFlags::empty(),
            dst_access_mask: ash::vk::AccessFlags::COLOR_ATTACHMENT_READ | ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: ash::vk::DependencyFlags::empty(),
        };

        let render_pass_create_info = ash::vk::RenderPassCreateInfo{
            s_type: ash::vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 1,
            p_dependencies: &subpass_dependencies
        };

        let render_pass1 = unsafe{device.create_render_pass(&render_pass_create_info, None).expect("could not create render pass")};

        RenderPass{handle_: render_pass1}
    }

    pub unsafe fn build_2d_render_pass(device: &ash::Device, surface_format: ash::vk::Format) -> RenderPass
    {
        let color_attachment = RenderPass::build_color_attachment_description(surface_format);

        let color_attachment_reference = ash::vk::AttachmentReference{
            attachment: 0,
            layout: ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let subpass = ash::vk::SubpassDescription{
            flags: ash::vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: ash::vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_reference,
            p_resolve_attachments: ptr::null(),
            p_depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        };

        let attachments = [color_attachment];

        let render_pass_create_info = ash::vk::RenderPassCreateInfo{
            s_type: ash::vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 0,
            p_dependencies: ptr::null()
        };

        let render_pass_handle = device.create_render_pass(&render_pass_create_info, None).expect("Could not create render pass");

        RenderPass{ handle_: render_pass_handle }
    }

    fn build_color_attachment_description(surface_format: ash::vk::Format) -> ash::vk::AttachmentDescription
    {
        ash::vk::AttachmentDescription{
            flags: ash::vk::AttachmentDescriptionFlags::empty(),
            format: surface_format,
            samples: ash::vk::SampleCountFlags::TYPE_1,
            load_op: ash::vk::AttachmentLoadOp::CLEAR,
            store_op: ash::vk::AttachmentStoreOp::STORE,
            stencil_load_op: ash::vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: ash::vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: ash::vk::ImageLayout::UNDEFINED,
            final_layout: ash::vk::ImageLayout::PRESENT_SRC_KHR,
        }
    }

    fn build_depth_attachment_description(depth_format: ash::vk::Format) -> ash::vk::AttachmentDescription
    {
        ash::vk::AttachmentDescription{
            flags: ash::vk::AttachmentDescriptionFlags::empty(),
            format: depth_format,
            samples: ash::vk::SampleCountFlags::TYPE_1,
            load_op: ash::vk::AttachmentLoadOp::CLEAR,
            store_op: ash::vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op: ash::vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: ash::vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: ash::vk::ImageLayout::UNDEFINED,
            final_layout: ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        }
    }



}
 */
