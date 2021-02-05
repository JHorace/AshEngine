use super::buffer;
use super::super::geometry;
use super::physical_device;
use super::command;
use super::super::geometry::palette::{Palette, MeshID};

use std::collections::HashMap;
use ash::vk::DeviceSize;
use std::os::raw::c_void;
use std::ptr;

use ash::version::DeviceV1_0;

const MAX_VERTICES: u64 = 10000;
const MAX_INDICES: u64 = 1000000;



pub struct MeshLocation
{
    pub vertex_offset_: usize,
    pub index_offset_: usize,
    pub vertex_count_: usize,
    pub index_count_: usize,
}

pub struct GeometryManager
{
    vertices_: Vec<geometry::vertex::Vertex>,
    pub indices_: Vec<u32>,
    mesh_locations_: HashMap<u64, MeshLocation>,
    vertex_staging_buffer_: buffer::Buffer,
    pub vertex_device_buffer_: buffer::Buffer,
    index_staging_buffer_: buffer::Buffer,
    pub index_device_buffer_: buffer::Buffer,
    curr_mesh_id_: u64,
    command_dispatch_: command::CommandDispatch,
    transfer_queue_: ash::vk::Queue,
    should_load_: bool,
}

impl GeometryManager
{
    pub fn new(instance: &ash::Instance, device: &ash::Device, physical_device: &physical_device::PhysicalDevice) -> GeometryManager
    {
        let vertex_buffer_size = (std::mem::size_of::<geometry::vertex::Vertex>() as u64 * MAX_VERTICES) as u64;
        let index_buffer_size = (std::mem::size_of::<u32>() as u64 * MAX_INDICES) as u64;

        let vertex_staging_buffer = buffer::Buffer::new(instance, device, physical_device, vertex_buffer_size, ash::vk::BufferUsageFlags::TRANSFER_SRC, ash::vk::MemoryPropertyFlags::HOST_VISIBLE | ash::vk::MemoryPropertyFlags::DEVICE_LOCAL);
        let vertex_device_buffer = buffer::Buffer::new(instance, device, physical_device, vertex_buffer_size, ash::vk::BufferUsageFlags::TRANSFER_DST | ash::vk::BufferUsageFlags::VERTEX_BUFFER, ash::vk::MemoryPropertyFlags::DEVICE_LOCAL);
        let index_staging_buffer = buffer::Buffer::new(instance, device, physical_device, index_buffer_size, ash::vk::BufferUsageFlags::TRANSFER_SRC, ash::vk::MemoryPropertyFlags::HOST_VISIBLE | ash::vk::MemoryPropertyFlags::DEVICE_LOCAL);
        let index_device_buffer = buffer::Buffer::new(instance, device, physical_device, index_buffer_size, ash::vk::BufferUsageFlags::TRANSFER_DST | ash::vk::BufferUsageFlags::INDEX_BUFFER, ash::vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let command_dispatch = unsafe{command::CommandDispatch::new(device, physical_device.queue_family_indices_.transfer_.unwrap(), 2) };
        let transfer_queue = unsafe { device.get_device_queue(physical_device.queue_family_indices_.transfer_.unwrap(), 0) };
        GeometryManager{
            vertices_: vec![],
            indices_: vec![],
            mesh_locations_: HashMap::new(),
            vertex_staging_buffer_: vertex_staging_buffer,
            vertex_device_buffer_: vertex_device_buffer,
            index_staging_buffer_: index_staging_buffer,
            index_device_buffer_: index_device_buffer,
            curr_mesh_id_: 0,
            command_dispatch_: command_dispatch,
            transfer_queue_: transfer_queue,
            should_load_: false,
        }
    }

    pub fn load_palette(& mut self, palette: &Palette)
    {
        for (mesh_id, mesh) in palette.meshes_.iter()
        {
            self.mesh_locations_.insert(*mesh_id, MeshLocation{
                vertex_offset_: self.vertices_.len(),
                index_offset_: self.indices_.len(),
                vertex_count_: mesh.vertices_.len(),
                index_count_: mesh.indices_.len(),
            });

            self.vertices_.extend(mesh.vertices_.iter().cloned());
            self.indices_.extend(mesh.indices_.iter().cloned());
        }

        self.should_load_ = true;
    }

    pub fn load_mesh(& mut self, mesh: &geometry::mesh::Mesh) -> u64
    {
        let mesh_id = self.curr_mesh_id_;

        self.mesh_locations_.insert(mesh_id, MeshLocation{
            vertex_offset_: self.vertices_.len(),
            index_offset_: self.indices_.len(),
            vertex_count_: mesh.vertices_.len(),
            index_count_: mesh.indices_.len(),
        });

        self.vertices_.extend(mesh.vertices_.iter().cloned());
        self.indices_.extend(mesh.indices_.iter().cloned());

        self.curr_mesh_id_ = self.curr_mesh_id_ + 1;

        self.should_load_ = true;

        mesh_id
    }

    pub fn get_mesh_location(&self, mesh_id: &u64) -> &MeshLocation
    {
        self.mesh_locations_.get(mesh_id).expect("No such mesh")
    }

    pub unsafe fn load_geometry_to_device(& mut self, device: &ash::Device)
    {
        self.stage_vertices(device);
        self.stage_indices(device);

        let vertex_copy_size = std::mem::size_of::<geometry::vertex::Vertex>() * self.vertices_.len();
        let index_copy_size = std::mem::size_of::<u32>() * self.indices_.len();
        let vertex_copy_info = ash::vk::BufferCopy{
            src_offset: 0,
            dst_offset: 0,
            size: vertex_copy_size as u64,
        };

        let index_copy_info = ash::vk::BufferCopy{
            src_offset: 0,
            dst_offset: 0,
            size: index_copy_size as u64,
        };

        let command_buffer1 = self.command_dispatch_.command_buffers_[0];
        let command_buffer2 = self.command_dispatch_.command_buffers_[1];
        self.vertex_device_buffer_.copy_from_buffer(device, self.transfer_queue_, command_buffer1, &self.vertex_staging_buffer_, vertex_copy_info);
        self.index_device_buffer_.copy_from_buffer(device, self.transfer_queue_, command_buffer2, &self.index_staging_buffer_, index_copy_info);
    }

    pub fn update(&mut self, device: &ash::Device)
    {
        if self.should_load_
        {
            unsafe{self.load_geometry_to_device(device)};
            self.should_load_ = false;
        }
    }

    unsafe fn stage_vertices(& mut self, device: &ash::Device)
    {
        self.vertex_staging_buffer_.map(device, ash::vk::DeviceSize::min_value(), ash::vk::WHOLE_SIZE);

        let copy_size = std::mem::size_of::<geometry::vertex::Vertex>() * self.vertices_.len();

        self.vertex_staging_buffer_.copy_from_data(self.vertices_.as_ptr() as * const c_void, copy_size as u64, ash::vk::DeviceSize::min_value());
        self.vertex_staging_buffer_.unmap(device);

    }

    unsafe fn stage_indices(& mut self, device: &ash::Device)
    {
        self.index_staging_buffer_.map(device, ash::vk::DeviceSize::min_value(), ash::vk::WHOLE_SIZE);

        let copy_size = std::mem::size_of::<u32>() * self.indices_.len();

        self.index_staging_buffer_.copy_from_data(self.indices_.as_ptr() as * const c_void, copy_size as u64, ash::vk::DeviceSize::min_value());
        self.index_staging_buffer_.unmap(device);
    }




}