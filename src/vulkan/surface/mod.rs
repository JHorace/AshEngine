use num;
use std::os::raw::c_void;
use std::os::raw::c_ulong;
use std::ptr;

use ash::version::{EntryV1_0, InstanceV1_0};

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;

pub struct Surface
{
    pub surface_loader_: ash::extensions::khr::Surface,
    pub surface_handle_: ash::vk::SurfaceKHR,
}

impl Surface
{
    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    pub unsafe fn new(entry: &ash::Entry, instance: &ash::Instance, display: * mut c_void, window: c_ulong) -> Surface
    {
        let surface_create_info = ash::vk::XlibSurfaceCreateInfoKHR{
            s_type: ash::vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: Default::default(),
            dpy: display as * mut ash::vk::Display,
            window: window as ash::vk::Window,
        };

        let xlib_surface_loader = XlibSurface::new(entry, instance);
        let surface_handle = xlib_surface_loader.create_xlib_surface(&surface_create_info, None).expect("Could not create surface");
        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        Surface{ surface_loader_: surface_loader, surface_handle_: surface_handle}
    }

    pub fn get_surface_capabilities(&self, physical_device: &ash::vk::PhysicalDevice) -> ash::vk::SurfaceCapabilitiesKHR
    {
        unsafe
            {
                self.surface_loader_.get_physical_device_surface_capabilities(*physical_device,
                                                                              self.surface_handle_)
                    .expect("Could not get surface capabilities")
            }
    }

    pub fn get_preferred_present_mode(&self, physical_device: &ash::vk::PhysicalDevice) -> ash::vk::PresentModeKHR
    {

        unsafe
            {
                let available_present_modes = self.surface_loader_
                    .get_physical_device_surface_present_modes(*physical_device, self.surface_handle_)
                    .expect("Could not get surface present modes");

                if available_present_modes.iter().any(|&i| i == ash::vk::PresentModeKHR::MAILBOX)
                {
                    ash::vk::PresentModeKHR::MAILBOX
                }
                else
                {
                    ash::vk::PresentModeKHR::FIFO
                }

            }
    }

    pub fn get_initial_extent(&self, physical_device: &ash::vk::PhysicalDevice, default_extent: ash::vk::Extent2D) -> ash::vk::Extent2D
    {
        let surface_capabilities = self.get_surface_capabilities(physical_device);

        if surface_capabilities.current_extent.width != u32::max_value()
        {
            surface_capabilities.current_extent
        }
        else
        {

            ash::vk::Extent2D{
                width: num::clamp(
                    default_extent.width,
                    surface_capabilities.min_image_extent.width,
                    surface_capabilities.max_image_extent.width,
                ),
                height: num::clamp(
                    default_extent.height,
                    surface_capabilities.min_image_extent.height,
                    surface_capabilities.max_image_extent.height,
                ),
            }
        }
    }

    pub fn get_min_image_count(&self, physical_device: &ash::vk::PhysicalDevice) -> u32
    {
        let surface_capabilities = self.get_surface_capabilities(physical_device);

        let mut min_image_count = surface_capabilities.min_image_count + 1;

        if (surface_capabilities.max_image_count > 0) && (min_image_count > surface_capabilities.max_image_count)
        {
            min_image_count = surface_capabilities.max_image_count;
        }

        min_image_count
    }

    pub fn get_preferred_image_format(&self, physical_device: &ash::vk::PhysicalDevice) -> ash::vk::SurfaceFormatKHR
    {
        unsafe
            {
                let surface_formats = self.surface_loader_
                    .get_physical_device_surface_formats(*physical_device, self.surface_handle_)
                    .expect("Could not get surface formats");

                if let Some(i) = surface_formats.iter().find(|&j|
                    (j.format == ash::vk::Format::B8G8R8A8_SRGB) && (j.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR) )
                {
                    i.clone()
                }
                else
                {
                    surface_formats.first().unwrap().clone()
                }
            }

    }

    pub fn get_current_transform(&self, physical_device: &ash::vk::PhysicalDevice) -> ash::vk::SurfaceTransformFlagsKHR
    {
        let surface_capabilities = self.get_surface_capabilities(physical_device);

        surface_capabilities.current_transform
    }
}
