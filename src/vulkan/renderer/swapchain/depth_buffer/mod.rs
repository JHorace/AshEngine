use crate::vulkan::physical_device;
use std::ptr;

use ash::version::DeviceV1_0;

pub struct DepthBuffer
{
    image_: ash::vk::Image,
    pub image_view_: ash::vk::ImageView,
    device_memory_: ash::vk::DeviceMemory,
}

impl DepthBuffer{
    pub fn new(instance: &ash::Instance, device: &ash::Device, physical_device: &physical_device::PhysicalDevice, extent: ash::vk::Extent2D) -> DepthBuffer
    {
        let format = physical_device.get_supported_format(instance,
                                                          &[ash::vk::Format::D32_SFLOAT, ash::vk::Format::D32_SFLOAT_S8_UINT, ash::vk::Format::D24_UNORM_S8_UINT, ],
                                                          ash::vk::ImageTiling::OPTIMAL,
                                                          ash::vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT).expect("Could not find supported format");


        let image_create_info = ash::vk::ImageCreateInfo{
            s_type: ash::vk::StructureType::IMAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::ImageCreateFlags::empty(),
            image_type: ash::vk::ImageType::TYPE_2D,
            format: format,
            extent: ash::vk::Extent3D{
                width: extent.width,
                height: extent.height,
                depth: 1
            },
            mip_levels: 1,
            array_layers: 1,
            samples: ash::vk::SampleCountFlags::TYPE_1,
            tiling: ash::vk::ImageTiling::OPTIMAL,
            usage: ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
            initial_layout: ash::vk::ImageLayout::UNDEFINED
        };

        let image = unsafe{device.create_image(&image_create_info, None).expect("Could not create depth image")};

        let memory_requirements = unsafe{device.get_image_memory_requirements(image)};

        let memory_allocate_info = ash::vk::MemoryAllocateInfo{
            s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: memory_requirements.size,
            memory_type_index: physical_device.get_suitable_memory_type_index(instance,
                                                                              memory_requirements.memory_type_bits,
                                                                              ash::vk::MemoryPropertyFlags::DEVICE_LOCAL)
                .expect("could not find suitable memory type")
        };

        let device_memory = unsafe{device.allocate_memory(&memory_allocate_info, None).expect("Could not allocate memory for depth image")};

        unsafe{
            device.bind_image_memory(image, device_memory, 0);
        }
        let image_view_create_info = ash::vk::ImageViewCreateInfo{
            s_type: ash::vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::ImageViewCreateFlags::empty(),
            image,
            view_type: ash::vk::ImageViewType::TYPE_2D,
            format,
            components: ash::vk::ComponentMapping{
                r: ash::vk::ComponentSwizzle::IDENTITY,
                g: ash::vk::ComponentSwizzle::IDENTITY,
                b: ash::vk::ComponentSwizzle::IDENTITY,
                a: ash::vk::ComponentSwizzle::IDENTITY
            },
            subresource_range: ash::vk::ImageSubresourceRange{
                aspect_mask: ash::vk::ImageAspectFlags::DEPTH,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            },
        };
        let image_view = unsafe{device.create_image_view(&image_view_create_info, None).expect("could not create depth image view")};

        DepthBuffer{
            image_: image,
            image_view_: image_view,
            device_memory_: device_memory,
        }
    }
}
