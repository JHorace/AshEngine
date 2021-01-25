use std::ptr;
use ash::version::DeviceV1_0;

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

        let render_pass = unsafe{device.create_render_pass(&render_pass_create_info, None).expect("could not create render pass")};

        RenderPass{handle_: render_pass}
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