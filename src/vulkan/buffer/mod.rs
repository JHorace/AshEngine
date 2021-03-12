use super::physical_device;
use ash::version::DeviceV1_0;
use std::os::raw::c_void;
use std::ptr;

pub struct Buffer {
    pub buffer_handle_: ash::vk::Buffer,
    device_memory_handle_: ash::vk::DeviceMemory,
    mapped_memory_: *mut c_void,
}

impl Buffer {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: &physical_device::PhysicalDevice,
        size: ash::vk::DeviceSize,
        usage: ash::vk::BufferUsageFlags,
        required_properties: ash::vk::MemoryPropertyFlags,
    ) -> Buffer {
        let buffer_create_info = ash::vk::BufferCreateInfo {
            s_type: ash::vk::StructureType::BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::BufferCreateFlags::empty(),
            size: size,
            usage: usage,
            sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };

        let buffer_handle = unsafe {
            device
                .create_buffer(&buffer_create_info, None)
                .expect("Could not create buffer")
        };

        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer_handle) };

        let memory_type_index = physical_device.get_suitable_memory_type_index(
            instance,
            memory_requirements.memory_type_bits,
            required_properties,
        );

        let memory_allocate_info = ash::vk::MemoryAllocateInfo {
            s_type: ash::vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: memory_requirements.size,
            memory_type_index: memory_type_index.unwrap(),
        };

        let device_memory_handle = unsafe {
            device
                .allocate_memory(&memory_allocate_info, None)
                .expect("Could not allocate device memory for buffer")
        };
        unsafe {
            device
                .bind_buffer_memory(buffer_handle, device_memory_handle, 0)
                .expect("Could not bind buffer memory")
        };

        Buffer {
            buffer_handle_: buffer_handle,
            device_memory_handle_: device_memory_handle,
            mapped_memory_: ptr::null_mut(),
        }
    }

    pub unsafe fn map(
        &mut self,
        device: &ash::Device,
        offset: ash::vk::DeviceSize,
        size: ash::vk::DeviceSize,
    ) -> *mut c_void {
        self.mapped_memory_ = device
            .map_memory(
                self.device_memory_handle_,
                offset,
                size,
                ash::vk::MemoryMapFlags::empty(),
            )
            .expect("Could not map memory");
        self.mapped_memory_
    }

    pub unsafe fn unmap(&self, device: &ash::Device) {
        device.unmap_memory(self.device_memory_handle_);
    }
    pub unsafe fn copy_from_data(
        &mut self,
        data: *const c_void,
        size: u64,
        offset: ash::vk::DeviceSize,
    ) {
        ptr::copy_nonoverlapping(
            data.offset(offset as isize),
            self.mapped_memory_,
            size as usize,
        );
    }

    pub unsafe fn copy_from_buffer(
        &mut self,
        device: &ash::Device,
        queue: ash::vk::Queue,
        command_buffer: ash::vk::CommandBuffer,
        src: &Buffer,
        copy_info: ash::vk::BufferCopy,
    ) {
        let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo {
            s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        let copy_infos = [copy_info];

        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Could not begin command buffer recording");
        device.cmd_copy_buffer(
            command_buffer,
            src.buffer_handle_,
            self.buffer_handle_,
            &copy_infos,
        );
        device
            .end_command_buffer(command_buffer)
            .expect("Could not end command buffer recording");

        let submit_info = [ash::vk::SubmitInfo {
            s_type: ash::vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 0,
            p_wait_semaphores: ptr::null(),
            p_wait_dst_stage_mask: ptr::null(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            signal_semaphore_count: 0,
            p_signal_semaphores: ptr::null(),
        }];

        device
            .queue_submit(queue, &submit_info, ash::vk::Fence::null())
            .expect("Could not submit to queue");
    }
}
