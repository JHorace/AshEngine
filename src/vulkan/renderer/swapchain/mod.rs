use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::super::physical_device;
use super::super::surface;

mod depth_buffer;

pub struct Swapchain
{
    swapchain_loader_: ash::extensions::khr::Swapchain,
    swapchain_handle_: ash::vk::SwapchainKHR,
    pub swapchain_images_: Vec<SwapchainImage>,
    depth_buffer_: depth_buffer::DepthBuffer,
    pub swapchain_framebuffers_: Vec<ash::vk::Framebuffer>,
    pub swapchain_extent_: ash::vk::Extent2D,
    pub swapchain_surface_format_ : ash::vk::Format,

}

pub struct SwapchainImage
{
    image_: ash::vk::Image,
    view_: ash::vk::ImageView,

}

impl Swapchain
{
    pub unsafe fn new(instance: &ash::Instance, device: &ash::Device, surface: &surface::Surface,
                      physical_device: &physical_device::PhysicalDevice) -> Swapchain
    {

        let surface_format = surface.get_preferred_image_format(&physical_device.physical_device_handle_);
        let present_mode = surface.get_preferred_present_mode(&physical_device.physical_device_handle_);
        let extent = surface.get_initial_extent(&physical_device.physical_device_handle_, ash::vk::Extent2D{width: 1920, height: 1080});
        let image_count = surface.get_min_image_count(&physical_device.physical_device_handle_);
        let image_sharing_mode = ash::vk::SharingMode::EXCLUSIVE;
        let current_transform = surface.get_current_transform(&physical_device.physical_device_handle_);

        let present_support = surface.surface_loader_.get_physical_device_surface_support(physical_device.physical_device_handle_, physical_device.queue_family_indices_.graphics_compute_.unwrap(), surface.surface_handle_);

        let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR{
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: ash::vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface.surface_handle_,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: ash::vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: image_sharing_mode,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            pre_transform: current_transform,
            composite_alpha: ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: present_mode,
            clipped: ash::vk::TRUE,
            old_swapchain: ash::vk::SwapchainKHR::null(),
        };

        let swapchain_loader = ash::extensions::khr::Swapchain::new(instance, device);
        unsafe
            {
                let swapchain_handle= swapchain_loader.create_swapchain(&swapchain_create_info, None).expect("Could not create Swapchain");

                let images = swapchain_loader.get_swapchain_images(swapchain_handle).expect("Could not get Swapchain images");
                let mut swapchain_images : Vec<SwapchainImage> = Vec::new();

                let mut image_view_create_info = ash::vk::ImageViewCreateInfo{
                    s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: vk::ImageViewCreateFlags::empty(),
                    image: Default::default(),
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: surface_format.format,
                    components: vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    },
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                };

                for image in images
                {
                    image_view_create_info.image = image;

                    swapchain_images.push(SwapchainImage{
                        image_: image,
                        view_: device.create_image_view(&image_view_create_info, None).expect("Could not create image view"),
                    });
                }

                let depth_buffer = depth_buffer::DepthBuffer::new(instance, device, physical_device, extent);

                
                Swapchain{
                    swapchain_loader_: swapchain_loader,
                    swapchain_handle_: swapchain_handle,
                    swapchain_images_: swapchain_images,
                    depth_buffer_: depth_buffer,
                    swapchain_framebuffers_: vec![],
                    swapchain_extent_: extent,
                    swapchain_surface_format_: surface_format.format,
                }
            }
    }

    pub unsafe fn create_swapchain_framebuffers(& mut self, device: &ash::Device, render_pass: ash::vk::RenderPass)
    {
        for image in self.swapchain_images_.iter()
        {
            let attachments = [image.view_, self.depth_buffer_.image_view_];

            let framebuffer_create_info = ash::vk::FramebufferCreateInfo{
                s_type: ash::vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: ptr::null(),
                flags: ash::vk::FramebufferCreateFlags::empty(),
                render_pass,
                attachment_count: attachments.len() as u32,
                p_attachments: attachments.as_ptr(),
                width: self.swapchain_extent_.width,
                height: self.swapchain_extent_.height,
                layers: 1
            };

            let framebuffer = device.create_framebuffer(&framebuffer_create_info, None).expect("Could not create framebuffer");

            self.swapchain_framebuffers_.push(framebuffer);
        }
    }

    pub unsafe fn acquire_next_image(&self, image_available_sempahore: ash::vk::Semaphore) -> (u32, bool)
    {
        self.swapchain_loader_.acquire_next_image(self.swapchain_handle_, std::u64::MAX, image_available_sempahore, ash::vk::Fence::null()).expect("could not acquire next image")
    }


    pub unsafe fn queue_present(&self, queue: ash::vk::Queue, image_index: u32, wait_semaphore: * const ash::vk::Semaphore)
    {
        let present_info = ash::vk::PresentInfoKHR{
            s_type: ash::vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.swapchain_handle_,
            p_image_indices: &image_index,
            p_results: ptr::null_mut()
        };

        self.swapchain_loader_.queue_present(queue, &present_info).expect("failed to queue present");

    }
}
