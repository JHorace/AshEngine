use std::ptr;
use ash::version::DeviceV1_0;

pub struct CommandDispatch
{
    command_pool_: ash::vk::CommandPool,
    pub command_buffers_: Vec<ash::vk::CommandBuffer>,
}

impl CommandDispatch{

    pub fn new(device: &ash::Device, submit_queue_family_index: u32, num_command_buffers: u32) -> CommandDispatch
    {
        let command_pool_create_info = ash::vk::CommandPoolCreateInfo{
            s_type: ash::vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: submit_queue_family_index,
        };

        let command_pool_handle = unsafe { device.create_command_pool(&command_pool_create_info, None) }.expect("could not create command pool");

        let command_buffers_allocate_info = ash::vk::CommandBufferAllocateInfo{
            s_type: ash::vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: command_pool_handle,
            level: ash::vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: num_command_buffers
        };

        let command_buffers = unsafe { device.allocate_command_buffers(&command_buffers_allocate_info) }.expect("Could not allocate command buffers");

        CommandDispatch{command_pool_: command_pool_handle, command_buffers_: command_buffers}
    }
}