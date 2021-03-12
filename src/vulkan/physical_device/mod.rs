use ash::vk;
use std::ffi::CStr;
use std::os::raw::c_char;

use super::utility::tools;
use crate::vulkan::utility::tools::truncate_optional_usize_to_optional_u32;
use ash::version::InstanceV1_0;
use std::ptr;

/// Stores information on the preferred device queue family indices for each operation type
pub struct QueueFamilyIndices {
    pub graphics_: Option<u32>,
    pub graphics_compute_: Option<u32>,
    pub compute_: Option<u32>,
    pub transfer_: Option<u32>,
}

impl QueueFamilyIndices {
    fn new(
        graphics: Option<usize>,
        graphics_compute: Option<usize>,
        compute: Option<usize>,
        transfer: Option<usize>,
    ) -> QueueFamilyIndices {
        let graphics_u32 = truncate_optional_usize_to_optional_u32(graphics);
        let graphics_compute_u32 = truncate_optional_usize_to_optional_u32(graphics_compute);
        let compute_u32 = truncate_optional_usize_to_optional_u32(compute);
        let transfer_u32 = truncate_optional_usize_to_optional_u32(transfer);

        QueueFamilyIndices {
            graphics_: graphics_u32,
            graphics_compute_: graphics_compute_u32,
            compute_: compute_u32,
            transfer_: transfer_u32,
        }
    }

    pub fn make_device_queue_create_infos(&self) -> Vec<vk::DeviceQueueCreateInfo> {
        let mut device_queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = Vec::new();
        let queue_priorities = [1.0_f32];
        if self.graphics_.is_some() {
            device_queue_create_infos.push(vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: self.graphics_.unwrap(),
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
            });
        }
        if self.graphics_compute_.is_some() {
            device_queue_create_infos.push(vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: self.graphics_compute_.unwrap(),
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
            });
        }
        if self.compute_.is_some() {
            device_queue_create_infos.push(vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: self.compute_.unwrap(),
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
            });
        }
        if self.transfer_.is_some() {
            device_queue_create_infos.push(vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: self.transfer_.unwrap(),
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
            });
        }

        device_queue_create_infos
    }
}

/// Contains capabilities, preferred presentation attributes, and the vulkan handle for a given
/// physical device. This allows physical devices and device capabilities to be enumerated and
/// selected by an application at runtime
pub struct PhysicalDevice {
    pub physical_device_handle_: vk::PhysicalDevice,
    pub device_name_: String,
    pub queue_family_indices_: QueueFamilyIndices,
}

impl PhysicalDevice {
    /// Creates a new PhysicalDeviceInfo containing physical device information, as well as the
    /// device's vulkan handle
    ///
    /// # Arguments
    ///
    /// * 'instance' - a reference to the vulkan instance, used to enumerate device capabilities
    ///
    /// * 'physical_device' - the vulkan physical device handle
    ///
    pub fn new(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> PhysicalDevice {
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = tools::c_char_array_to_string(&device_properties.device_name);
        let queue_family_indices =
            PhysicalDevice::select_queue_family_indices(instance, physical_device);
        PhysicalDevice {
            physical_device_handle_: physical_device,
            device_name_: device_name,
            queue_family_indices_: queue_family_indices,
        }
    }

    fn select_queue_family_indices(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let transfer = queue_families.iter().position(|&queue_family| {
            queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER)
                && !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        });

        let compute = queue_families.iter().position(|&queue_family| {
            queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE)
                && !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        });

        let graphics = queue_families.iter().position(|&queue_family| {
            queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                && !queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE)
        });

        let graphics_compute = queue_families.iter().position(|&queue_family| {
            queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                && queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE)
        });

        QueueFamilyIndices::new(graphics, graphics_compute, compute, transfer)
    }

    pub fn get_suitable_memory_type_index(
        &self,
        instance: &ash::Instance,
        type_bitmask: u32,
        require_properties: ash::vk::MemoryPropertyFlags,
    ) -> Option<u32> {
        let device_memory_properties =
            unsafe { instance.get_physical_device_memory_properties(self.physical_device_handle_) };

        for (i, memory_type) in device_memory_properties.memory_types.iter().enumerate() {
            if (type_bitmask & (1 << i)) > 0
                && (memory_type.property_flags.contains(require_properties))
            {
                return Some(i as u32);
            }
        }
        None
    }

    pub fn get_supported_format(
        &self,
        instance: &ash::Instance,
        candidate_formats: &[ash::vk::Format],
        tiling: ash::vk::ImageTiling,
        features: ash::vk::FormatFeatureFlags,
    ) -> Option<ash::vk::Format> {
        for &format in candidate_formats.iter() {
            let properties = unsafe {
                instance.get_physical_device_format_properties(self.physical_device_handle_, format)
            };

            if tiling == ash::vk::ImageTiling::LINEAR
                && properties.linear_tiling_features.contains(features)
            {
                return Some(format.clone());
            } else if tiling == ash::vk::ImageTiling::OPTIMAL
                && properties.optimal_tiling_features.contains(features)
            {
                return Some(format.clone());
            }
        }
        None
    }
}

/// Returns a list of PhysicalDeviceInfo representing physical devices capable of rendering. A
/// suitable device must be a GPU capable of presenting to given surface types, that
/// supports all device extensions required by the application
///
/// # Arguments
///
/// * 'instance' - a reference to the vulkan instance, used to enumerate physical devices and device
/// capabilities
///
/// * 'required_extensions' - a list of device extensions required by the application
///
pub fn get_suitable_physical_devices(
    instance: &ash::Instance,
    num_device_extensions: &u32,
    device_extensions: &*const *const c_char,
) -> Vec<PhysicalDevice> {
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices")
    };
    let mut suitable_devices: Vec<PhysicalDevice> = Vec::new();
    for device in physical_devices {
        if is_suitable(instance, device, num_device_extensions, device_extensions) {
            suitable_devices.push(PhysicalDevice::new(instance, device));
        }
    }

    suitable_devices
}

/// Returns true if a given physical device is a GPU
///
/// # Arguments
///
/// * 'instance' - a reference to the vulkan instance, used to enumerate device properties
///
/// * 'physical_device' - the vulkan physical device handle
///
fn is_gpu(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> bool {
    let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };

    match device_properties.device_type {
        vk::PhysicalDeviceType::DISCRETE_GPU
        | vk::PhysicalDeviceType::INTEGRATED_GPU
        | vk::PhysicalDeviceType::VIRTUAL_GPU => true,
        _ => false,
    }
}

/// Returns true if a given physical device supports the requested device extensions
///
/// # Arguments
///
/// * 'instance' - a reference to the vulkan instance, used to enumerate supported device extensions
///
/// * 'physical_device' - the vulkan physical device handle
///
fn supports_extensions(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    num_device_extensions: &u32,
    device_extensions: &*const *const c_char,
) -> bool {
    let available_device_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(physical_device)
            .expect("Could not enumeration device extension properties")
    };

    for i in 0..*num_device_extensions {
        unsafe {
            if !available_device_extensions.iter().any(|&j| {
                CStr::from_ptr(j.extension_name.as_ptr())
                    == CStr::from_ptr(*device_extensions.offset(i as isize))
            }) {
                return false;
            }
        }
    }
    true
}

/// Returns true if a given physical device supports graphics and presentation
///
/// # Arguments
///
/// * 'instance' - a reference to the vulkan instance, used to enumerate queue family properties
///
/// * 'physical_device' - the vulkan physical device handle
///
fn supports_graphics_and_presentation(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let device_queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    device_queue_families
        .iter()
        .any(|&i| i.queue_flags.contains(vk::QueueFlags::GRAPHICS))
}

/// Determines whether a physical device is suitable for the engine and application. A device is
/// suitable if it is a GPU that supports graphics and presentation and all requested device
/// extensions.
///
/// # Arguments
///
/// * 'instance' - a reference to the vulkan instance, used to enumerate device properties
///
/// * 'physical_device' - the vulkan physical device handle
///
/// * 'required_extensions' - a list of device extensions required by the engine and application
///
fn is_suitable(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    num_device_extensions: &u32,
    device_extensions: &*const *const c_char,
) -> bool {
    return is_gpu(instance, physical_device)
        && supports_graphics_and_presentation(instance, physical_device)
        && supports_extensions(
            instance,
            physical_device,
            num_device_extensions,
            device_extensions,
        );
}
