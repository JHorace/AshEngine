use ash::Device;
use ash::vk::Queue;
use ash::vk;
use std::ptr;
use std::os::raw::c_char;
use super::physical_device::PhysicalDevice;
use ash::version::InstanceV1_0;
use ash::version::DeviceV1_0;
pub struct LogicalDevice
{
    pub device_: Device,
    graphics_queue_: Queue,
}

impl LogicalDevice
{

    pub fn new(instance: &ash::Instance, physical_device: &PhysicalDevice,
               num_validation_layers: &u32, validation_layers: &* const * const c_char,
               num_extension_names: &u32, extension_names: &* const * const c_char) -> LogicalDevice
    {
        let device_queue_create_infos = physical_device.queue_family_indices_.make_device_queue_create_infos();

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: device_queue_create_infos.len() as u32,

            p_queue_create_infos: device_queue_create_infos.as_ptr(),
            enabled_layer_count: *num_validation_layers,
            pp_enabled_layer_names: *validation_layers,
            enabled_extension_count: *num_extension_names,
            pp_enabled_extension_names: *extension_names,
            p_enabled_features: ptr::null(),
        };

        let device= unsafe{instance.create_device(physical_device.physical_device_handle_,
                                                  &device_create_info, None)
        .expect("Failed to create logical device")};

        let graphics_queue = unsafe { device.get_device_queue(physical_device.queue_family_indices_.graphics_compute_.unwrap(), 0)};

        LogicalDevice{device_: device, graphics_queue_: graphics_queue}

    }


}