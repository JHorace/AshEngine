use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_ulong;
use std::os::raw::c_void;
use std::ptr;

use ash::vk;
use ash::version::EntryV1_0;
use ash::vk::make_version;

use utility::debug;

use crate::vulkan::geometry_manager::GeometryManager;
use crate::vulkan::logical_device::LogicalDevice;
use crate::vulkan::renderer::Renderer;
use crate::vulkan::utility::debug::build_debug_messenger_create_info;
use crate::vulkan::utility::platforms::required_extension_names;

use super::geometry;

use super::geometry::palette::Palette;

mod utility;
mod physical_device;
mod logical_device;
pub mod renderer;
mod surface;
mod pipeline;
mod buffer;
mod geometry_manager;
mod command;

/// Used to configure a vulkan::Engine on Engine creation
pub struct EngineCreateInfo
{
    /// The name of the application using this engine
    pub app_name_: CString,
    /// The version of the application using this engine
    pub app_version_: u32,
    pub num_validation_layers_: u32,
    pub validation_layers_: * const * const c_char,
    pub num_device_extension_names_: u32,
    pub device_extension_names_: * const * const c_char,
}

/// Manages all aspects of the vulkan context, and contains functionality for rendering to multiple
/// windows. Creating an Engine will initialize the vulkan context. Destroying an Engine will delete
/// all vulkan resources, but will not destroy any windows or external geometry data.
pub struct Engine
{
    /// vulkan function loader
    entry_: ash::Entry,
    /// vulkan instance, configured to support requested validation layers and instance extensions
    instance_: ash::Instance,
    /// a list of physical devices suitable for rendering
    physical_devices_: Vec<physical_device::PhysicalDevice>,

    logical_device_: LogicalDevice,

    pub geometry_manager_: GeometryManager,
    
    pub renderers_: Vec<renderer::Renderer>,
    /// optional validation layers and debug messenger for handling validation messages
    debug_: Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>,
}

impl Engine
{
    /// Creates a new vulkan engine, configured to support requested validation layers and instance
    /// extensions. Panics if any part of Engine creation fails.
    ///
    /// # Arguments
    ///
    /// * 'create_info' - contains all data necessary to configure the Engine
    ///
    pub fn new(create_info: &EngineCreateInfo) -> Engine
    {
        // Load Vulkan entry functions. Panic if vulkan functions cannot be loaded
        let entry = ash::Entry::new().expect("Could not load vulkan functions");
        // Configure and create the vulkan instance. Panic if instance cannot be created
        let instance = Engine::create_instance(&entry, &create_info);
        // Enable validation if requested. Panic if the debug messenger cannot be created
        let debug = Engine::enable_validation(&entry, &instance, &create_info.num_validation_layers_, &create_info.validation_layers_);
        // Get a vector of suitable physical devices. Panic if devices or device properties cannot
        // be enumerated
        let physical_devices = physical_device::get_suitable_physical_devices(&instance, &create_info.num_device_extension_names_, &create_info.device_extension_names_);
        // Create a logical device from the first suitable physical device
        let logical_device = logical_device::LogicalDevice::new(&instance,
                                                                physical_devices.first().unwrap(),
                                                                &create_info.num_validation_layers_, &create_info.validation_layers_,
                                                                &create_info.num_device_extension_names_, &create_info.device_extension_names_);

        let geometry_manager = GeometryManager::new(&instance, &logical_device.device_, physical_devices.first().unwrap());

        Engine{entry_: entry, instance_: instance, physical_devices_: physical_devices, logical_device_: logical_device, geometry_manager_: geometry_manager, renderers_: vec![], debug_:debug}
    }
    
    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    pub unsafe fn create_renderer(&mut self, display: * mut c_void, window: c_ulong) -> u32
    {
        let surface = surface::Surface::new(&self.entry_, &self.instance_, display, window);

        let renderer = renderer::Renderer::new(&self.instance_, &self.logical_device_.device_, &self.physical_devices_.first().unwrap(), &surface);

        self.renderers_.push(renderer);

        self.renderers_.len() as u32 - 1
    }

    pub fn get_renderer(& self, renderer_id: u32) -> &Renderer
    {
        self.renderers_.get(renderer_id as usize).unwrap()
    }

    pub fn set_renderer_pipeline(& mut self, renderer_id: u32, pipeline: pipeline::Pipeline, render_pass: pipeline::render_pass::RenderPass, descriptor_set_layout: ash::vk::DescriptorSetLayout)
    {
        unsafe { self.renderers_.get_mut(renderer_id as usize).unwrap().set_pipeline(&self.logical_device_.device_, pipeline, render_pass, descriptor_set_layout, &self.geometry_manager_); }
    }

    pub fn set_forward_rendering(&mut self, renderer_handle: u32, vert_path: &str, frag_path: &str)
    {
        let descriptor_set_layout_bindings = unsafe{pipeline::descriptor_set_layout::build_descriptor_set_layout_bindings()};
        let descriptor_set_layout = unsafe{pipeline::descriptor_set_layout::DescriptorSetLayout::new(&self.logical_device_.device_, descriptor_set_layout_bindings.to_vec()) };
        let renderer = unsafe{self.get_renderer(renderer_handle)};
        let depth_format = unsafe{self.physical_devices_.first().unwrap().get_supported_format(&self.instance_,
                                                                                                 &[ash::vk::Format::D32_SFLOAT, ash::vk::Format::D32_SFLOAT_S8_UINT, ash::vk::Format::D24_UNORM_S8_UINT],
                                                                                                 ash::vk::ImageTiling::OPTIMAL,
                                                                                                 ash::vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT).expect("could not find supported depth format") };
        let render_pass = unsafe {pipeline::render_pass::RenderPass::build_forward_render_pass(&self.logical_device_.device_, renderer.swapchain_.swapchain_surface_format_, depth_format) };
        let pipeline = unsafe{pipeline::Pipeline::new_forward_graphics_pipeline(&self.logical_device_.device_, &descriptor_set_layout.layout_handle_, render_pass.handle_, vert_path, frag_path) };

        self.set_renderer_pipeline(renderer_handle, pipeline, render_pass, descriptor_set_layout.layout_handle_)
    }

    pub fn update(&mut self)
    {
        self.geometry_manager_.update(&self.logical_device_.device_);

        for renderer in self.renderers_.iter_mut()
        {
            unsafe { renderer.update(&self.logical_device_.device_, &self.geometry_manager_); }
        }

    }

    pub fn load_palette(& mut self, palette: &Palette)
    {
        self.geometry_manager_.load_palette(palette);
    }

    /// Creates a vulkan instance configured with requested extension and validation support
    ///
    /// # Arguments
    ///
    /// * 'entry' - vulkan function loader used to create the instance
    ///
    /// * 'create_info' - contains all data necessary to configure the Engine
    ///
    fn create_instance(entry: &ash::Entry, create_info: &EngineCreateInfo) -> ash::Instance
    {
        let application_info = vk::ApplicationInfo
        {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: create_info.app_name_.as_ptr(),
            application_version: create_info.app_version_,
            p_engine_name: CString::new(env!("CARGO_PKG_NAME")).expect("Could not find cargo package name").as_ptr(),
            engine_version: make_version(1, 0, 0),
            api_version: make_version(1, 0, 0),
        };

        let debug_create_info = build_debug_messenger_create_info();

        let instance_extension_names = required_extension_names();

        let create_info = vk::InstanceCreateInfo
        {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: match create_info.num_validation_layers_
            {
                0u32 => ptr::null(),
                _ => &build_debug_messenger_create_info() as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const c_void,
            },
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &application_info,
            pp_enabled_layer_names: create_info.validation_layers_,
            enabled_layer_count: create_info.num_validation_layers_,
            pp_enabled_extension_names: instance_extension_names.as_ptr(),
            enabled_extension_count: instance_extension_names.len() as u32,
        };

        unsafe {entry.create_instance(&create_info, None).expect("Failed to create instance")}
    }

    /// Enables validation layers if validation is requested. Fails silently if validation is not
    /// requested, or if the instance does not support the requested validation layers
    ///
    /// # Arguments
    ///
    /// * 'validation_enabled' - whether validation layers should be enabled
    ///
    /// * 'instance' - the vulkan instance validation will be enabled for
    ///
    fn enable_validation(entry: &ash::Entry, instance: &ash::Instance, num_validation_layers: &u32, validation_layers: &* const * const c_char) -> Option<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)>
    {
        if *num_validation_layers != 0u32 &&
            debug::check_validation_layer_support(&entry, num_validation_layers, validation_layers)
        {
            Some(debug::setup_debug_utils(entry, instance))
        }
        else
        {

            None
        }
    }

}

#[cfg(test)]
mod tests
{

}
