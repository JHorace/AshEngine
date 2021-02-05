use std::f32::MAX;
use std::ptr;

use ash::version::DeviceV1_0;

use crate::vulkan::command;

use super::buffer;
use super::physical_device;
use super::pipeline;
use super::surface;
use super::geometry_manager;
use super::pipeline::{render_pass::RenderPass, Pipeline};
use super::super::geometry::scene::{Scene, Instance};
use super::super::geometry::palette::{MeshID};

use cgmath::Matrix4;
use cgmath::conv;

use std::mem::transmute;
use std::mem::size_of;
use std::collections::VecDeque;
use ash::vk::DescriptorBufferInfo;

mod swapchain;
mod descriptors;
mod uniform_manager;
pub mod instance_manager;
pub mod scene_manager;

const MAX_FRAMES_IN_FLIGHT: u32 = 2;
const MAX_INSTANCES: u32 = 100;
const MAX_LIGHTS: u32 = 100;

pub struct VulkanInstance
{
    pub mesh_id_: MeshID,
    pub descriptor_set_: ash::vk::DescriptorSet
}

pub struct FrameData
{
    pub vulkan_instances_: Vec<VulkanInstance>,
    pub view_: Matrix4<f32>,
    pub projection_: Matrix4<f32>,
}

pub struct Renderer
{
    pub swapchain_: swapchain::Swapchain,
    pub instance_manager_: instance_manager::InstanceManager,
    pub scene_manager_: scene_manager::SceneManager,
    scenes_: VecDeque<Scene>,
    uniform_manager_: uniform_manager::UniformManager,
    command_dispatch_: command::CommandDispatch,
    present_queue_: ash::vk::Queue,
    descriptor_pool_: ash::vk::DescriptorPool,
    descriptor_sets_: Vec<ash::vk::DescriptorSet>,
    render_finished_semaphores_: Vec<ash::vk::Semaphore>,
    image_available_sempahores_: Vec<ash::vk::Semaphore>,
    in_flight_fences_: Vec<ash::vk::Fence>,
    current_frame_: u32,
    curr_descriptor_set_: u64,
    instances_loaded_: bool,
    pipeline_: Option<Pipeline>,
    render_pass_: Option<RenderPass>,
}

impl Renderer
{
    pub unsafe fn new(instance: &ash::Instance, device: &ash::Device, physical_device: &physical_device::PhysicalDevice, surface: &surface::Surface) -> Renderer
    {
        let swapchain = swapchain::Swapchain::new(instance, device, surface, physical_device);

        let instance_manager = instance_manager::InstanceManager::new(instance, device, physical_device,  MAX_INSTANCES as u64, swapchain.swapchain_images_.len());

        let scene_manager = scene_manager::SceneManager::new(instance, device, physical_device, MAX_LIGHTS as u64, swapchain.swapchain_images_.len());

        let uniform_manager = uniform_manager::UniformManager::new(instance, device, physical_device, 100, MAX_FRAMES_IN_FLIGHT as u32, size_of::<Matrix4<f32>>() as u64);

        let command_dispatch = command::CommandDispatch::new(device, physical_device.queue_family_indices_.graphics_compute_.unwrap(), swapchain.swapchain_images_.len() as u32);

        let pool_sizes = [ash::vk::DescriptorPoolSize{
            ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: swapchain.swapchain_images_.len() as u32 * MAX_INSTANCES,
        }];

        let descriptor_pool_create_info = ash::vk::DescriptorPoolCreateInfo{
            s_type: ash::vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::DescriptorPoolCreateFlags::empty(),
            max_sets: swapchain.swapchain_images_.len() as u32 * MAX_INSTANCES,
            pool_size_count: pool_sizes.len() as u32,
            p_pool_sizes: pool_sizes.as_ptr(),
        };

        let descriptor_pool = unsafe{device.create_descriptor_pool(&descriptor_pool_create_info, None).expect("Could not create descriptor pool")};

        let semaphore_create_info = ash::vk::SemaphoreCreateInfo{
            s_type: ash::vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::SemaphoreCreateFlags::empty(),
        };

        let fence_create_info = ash::vk::FenceCreateInfo{
            s_type: ash::vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::FenceCreateFlags::SIGNALED
        };

        let mut render_finished_semaphores = vec![];
        let mut image_ready_semaphores = vec![];
        let mut in_flight_fences = vec![];

        for i in 0..MAX_FRAMES_IN_FLIGHT
        {
            render_finished_semaphores.push(device.create_semaphore(&semaphore_create_info, None).expect("Could not create semaphore"));
            image_ready_semaphores.push(device.create_semaphore(&semaphore_create_info, None).expect("Could not create semaphore"));
            in_flight_fences.push(device.create_fence(&fence_create_info, None).expect("Could not create fence"));
        }

        let present_queue = device.get_device_queue(physical_device.queue_family_indices_.graphics_compute_.unwrap(), 0);

        Renderer{swapchain_: swapchain, instance_manager_: instance_manager, scene_manager_: scene_manager, scenes_: VecDeque::new(), uniform_manager_: uniform_manager, command_dispatch_: command_dispatch, present_queue_: present_queue, descriptor_pool_: descriptor_pool, descriptor_sets_: vec![], render_finished_semaphores_: render_finished_semaphores, image_available_sempahores_: image_ready_semaphores, in_flight_fences_: in_flight_fences, current_frame_: 0, curr_descriptor_set_: 0, instances_loaded_: true, pipeline_: None, render_pass_: None }
    }

    pub unsafe fn set_pipeline(&mut self, device: &ash::Device, pipeline: pipeline::Pipeline, render_pass: pipeline::render_pass::RenderPass, descriptor_set_layout: ash::vk::DescriptorSetLayout, geometry_manager: &geometry_manager::GeometryManager)
    {
        self.swapchain_.create_swapchain_framebuffers(device, render_pass.handle_);

        self.create_descriptor_sets(device, descriptor_set_layout);

        self.render_pass_ = Some(render_pass);
        self.pipeline_ = Some(pipeline);
        //self.record_draw_commands_forward(device, render_pass.handle_, pipeline.pipeline_handle_, pipeline.layout_.layout_handle_, geometry_manager);

    }

    pub fn create_instance(&mut self, mesh_id: u64) -> u64
    {
        let instance_descriptor_set = self.curr_descriptor_set_;
        self.curr_descriptor_set_ = self.curr_descriptor_set_ + 1;
        self.instances_loaded_ = false;
        self.instance_manager_.create_instance(mesh_id, self.descriptor_sets_[instance_descriptor_set as usize])
    }

    fn create_descriptor_sets(&mut self, device: &ash::Device, descriptor_set_layout: ash::vk::DescriptorSetLayout)
    {
        let mut layouts : Vec<ash::vk::DescriptorSetLayout> = vec![];

        for _ in 0..self.swapchain_.swapchain_images_.len()
        {
            for _ in 0..MAX_INSTANCES
            {
                layouts.push(descriptor_set_layout);
            }

        }

        let descriptor_set_allocate_info = ash::vk::DescriptorSetAllocateInfo{
            s_type: ash::vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool: self.descriptor_pool_,
            descriptor_set_count: self.swapchain_.swapchain_images_.len() as u32,
            p_set_layouts: layouts.as_ptr()
        };


        self.descriptor_sets_ = unsafe{device.allocate_descriptor_sets(&descriptor_set_allocate_info).expect("Could not allocate descriptor sets")};
    }
/*
    fn update_descriptor_sets(&self, device: &ash::Device, curr_frame: u32)
    {
        let instances = self.instance_manager_.get_instances();

        let mut write_descriptor_sets = vec![];

        let light_buffer_descriptor_info = self.scene_manager_.get_descriptor_buffer_info();

        for (instance, buffer_descriptor_info) in instances.iter()
        {
            write_descriptor_sets.push(ash::vk::WriteDescriptorSet{
                s_type: ash::vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: instance.descriptor_set_,
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: ptr::null(),
                p_buffer_info: buffer_descriptor_info,
                p_texel_buffer_view: ptr::null()
            });
            write_descriptor_sets.push(ash::vk::WriteDescriptorSet{
                s_type: ash::vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: instance.descriptor_set_,
                dst_binding: 1,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: ptr::null(),
                p_buffer_info: &light_buffer_descriptor_info,
                p_texel_buffer_view: ptr::null()
            });
        }

        unsafe { device.update_descriptor_sets(write_descriptor_sets.as_ref(), &[]) };
    }
*/
    pub fn update(&mut self, device: &ash::Device, geometry_manager: &geometry_manager::GeometryManager)
    {
        let curr_scene = self.scenes_.pop_front();

        match curr_scene
        {
            Some(scene) => {

                let frame_data = FrameData{
                    vulkan_instances_: self.process_scene(device, &scene),
                    view_: scene.view_,
                    projection_: scene.projection_
                };

                self.record_draw_commands_forward(device, geometry_manager, frame_data);
            },

            None => {}
        };



        let wait_fences = [self.in_flight_fences_[self.current_frame_ as usize]];

        unsafe {
            device.wait_for_fences(&wait_fences, true, std::u64::MAX);

            let (image_index, ready) = self.swapchain_.acquire_next_image(self.image_available_sempahores_[self.current_frame_ as usize]);

            let wait_semaphores = [self.image_available_sempahores_[self.current_frame_ as usize]];
            let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal_semaphores = [self.render_finished_semaphores_[self.current_frame_ as usize]];

            let submit_infos = [ash::vk::SubmitInfo {
                s_type: ash::vk::StructureType::SUBMIT_INFO,
                p_next: ptr::null(),
                wait_semaphore_count: wait_semaphores.len() as u32,
                p_wait_semaphores: wait_semaphores.as_ptr(),
                p_wait_dst_stage_mask: wait_stages.as_ptr(),
                command_buffer_count: 1,
                p_command_buffers: &self.command_dispatch_.command_buffers_[image_index as usize],
                signal_semaphore_count: signal_semaphores.len() as u32,
                p_signal_semaphores: signal_semaphores.as_ptr()
            }];

            device.reset_fences(&wait_fences).expect("could not reset fence");

            device.queue_submit(self.present_queue_, &submit_infos, self.in_flight_fences_[self.current_frame_ as usize]);

            self.swapchain_.queue_present(self.present_queue_, image_index, signal_semaphores[0]);

            self.current_frame_ = (self.current_frame_ + 1) % MAX_FRAMES_IN_FLIGHT;
        }

    }

    pub fn queue_scene(&mut self, scene: &Scene)
    {
        self.scenes_.push_back(scene.clone());
    }

    fn process_scene(&mut self, device: &ash::Device, scene: &Scene) -> Vec<VulkanInstance>
    {
        let mut vulkan_instances = vec![];
        let instances = scene.get_instances();

        let descriptor_buffer_infos = self.uniform_manager_.update_uniforms(self.current_frame_, &instances);

        let descriptor_sets = self.update_descriptor_sets(device, &descriptor_buffer_infos);

        for (i, instance) in instances.iter().enumerate()
        {
            vulkan_instances.push(VulkanInstance{ mesh_id_: instance.mesh_id_, descriptor_set_: descriptor_sets[i] })
        }

        vulkan_instances
    }

    fn update_descriptor_sets(&mut self, device: &ash::Device, descriptor_buffer_infos: &Vec<DescriptorBufferInfo>) -> Vec<ash::vk::DescriptorSet>
    {
        let mut descriptor_sets = vec![];

        let mut write_descriptor_sets = vec![];

        for (i, descriptor_buffer_info) in descriptor_buffer_infos.iter().enumerate()
        {
            descriptor_sets.push(self.descriptor_sets_[i]);
            write_descriptor_sets.push(ash::vk::WriteDescriptorSet{
                s_type: ash::vk::StructureType::WRITE_DESCRIPTOR_SET,
                p_next: ptr::null(),
                dst_set: descriptor_sets[i],
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
                p_image_info: ptr::null(),
                p_buffer_info: descriptor_buffer_info,
                p_texel_buffer_view: ptr::null()
            });
        }

        unsafe { device.update_descriptor_sets(write_descriptor_sets.as_ref(), &[]) };

        descriptor_sets
    }

/*
    pub unsafe fn draw_frame(&mut self, device: &ash::Device, geometry_manager: &geometry_manager::GeometryManager)
    {
        self.instance_manager_.update(self.current_frame_ as u32);
        self.scene_manager_.update(self.current_frame_ as u32);
        if !self.instances_loaded_
        {
            self.update_descriptor_sets(device, self.current_frame_);
            self.record_draw_commands_forward(device, geometry_manager);
            self.instances_loaded_ = true;
        }



        let wait_fences = [self.in_flight_fences_[self.current_frame_ as usize]];

        device.wait_for_fences(&wait_fences, true, std::u64::MAX);

        let (image_index, ready) = self.swapchain_.acquire_next_image(self.image_available_sempahores_[self.current_frame_ as usize]);

        let wait_semaphores = [self.image_available_sempahores_[self.current_frame_ as usize]];
        let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.render_finished_semaphores_[self.current_frame_ as usize]];

        let submit_infos = [ash::vk::SubmitInfo{
            s_type: ash::vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_dispatch_.command_buffers_[image_index as usize],
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr()
        }];
        
        device.reset_fences(&wait_fences).expect("could not reset fence");
        
        device.queue_submit(self.present_queue_, &submit_infos, self.in_flight_fences_[self.current_frame_ as usize]);
        
        self.swapchain_.queue_present(self.present_queue_, image_index, signal_semaphores[0]);
        
        self.current_frame_ = (self.current_frame_ + 1) % MAX_FRAMES_IN_FLIGHT;

    }
*/
    unsafe fn record_draw_commands_2d_hard(&mut self, device: &ash::Device, render_pass: ash::vk::RenderPass, pipeline: ash::vk::Pipeline)
    {
        for(i, &command_buffer) in self.command_dispatch_.command_buffers_.iter().enumerate()
        {
            let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo{
                s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                flags: ash::vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
                p_inheritance_info: ptr::null()
            };

            device.begin_command_buffer(command_buffer, &command_buffer_begin_info).expect("could not begin recording command buffer");

            let clear_values = [ash::vk::ClearValue{
                color: ash::vk::ClearColorValue{
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = ash::vk::RenderPassBeginInfo{
                s_type: ash::vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: render_pass,
                framebuffer: self.swapchain_.swapchain_framebuffers_[i],
                render_area: ash::vk::Rect2D{
                    offset: ash::vk::Offset2D{x: 0, y: 0},
                    extent: self.swapchain_.swapchain_extent_
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };

            device.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, ash::vk::SubpassContents::INLINE);

            device.cmd_bind_pipeline(command_buffer, ash::vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer).expect("Could not end recording command buffer");
        }
    }

    pub fn mat4_to_bytes(matrix: Matrix4<f32>) -> Vec<u8>
    {
        let mut bytes = vec![];

        let mat_array = conv::array4x4(matrix);

        for i in 0..4
        {
            for j in 0..4
            {
                let element_bytes = mat_array[i][j].to_le_bytes();
                bytes.extend_from_slice(&element_bytes);
            }
        }


        bytes
    }

    fn record_draw_commands_forward(&mut self, device: &ash::Device, geometry_manager: &geometry_manager::GeometryManager, frame_data: FrameData)
    {
        for(i, &command_buffer) in self.command_dispatch_.command_buffers_.iter().enumerate()
        {
            let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo{
                s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                flags: ash::vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
                p_inheritance_info: ptr::null()
            };

            unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info).expect("could not begin recording command buffer"); }

            let clear_values = [
                ash::vk::ClearValue{
                    color: ash::vk::ClearColorValue{
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }},
                ash::vk::ClearValue{
                    depth_stencil: ash::vk::ClearDepthStencilValue{
                        depth: 1.0,
                        stencil: 0,
                    },
                }];

            let render_pass_begin_info = ash::vk::RenderPassBeginInfo{
                s_type: ash::vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: self.render_pass_.unwrap().handle_,
                framebuffer: self.swapchain_.swapchain_framebuffers_[i],
                render_area: ash::vk::Rect2D{
                    offset: ash::vk::Offset2D{x: 0, y: 0},
                    extent: self.swapchain_.swapchain_extent_
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };
            unsafe{
                device.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, ash::vk::SubpassContents::INLINE);

                let mut push_constant_vec = vec![];

                let mut view_bytes = Renderer::mat4_to_bytes(frame_data.view_);

                let mut projection_bytes = Renderer::mat4_to_bytes(frame_data.projection_);

                push_constant_vec.append(& mut view_bytes);
                push_constant_vec.append(& mut projection_bytes);


                let push_constant = 2u32;

                let bytes : [u8; 4] = push_constant.to_le_bytes();

                device.cmd_push_constants(
                    command_buffer,
                    self.pipeline_.unwrap().layout_.layout_handle_,
                    ash::vk::ShaderStageFlags::FRAGMENT,
                    0,
                    push_constant_vec.as_slice()
                    ,
                );

                device.cmd_bind_pipeline(command_buffer, ash::vk::PipelineBindPoint::GRAPHICS, self.pipeline_.unwrap().pipeline_handle_);

                let vertex_buffers = [geometry_manager.vertex_device_buffer_.buffer_handle_];
                let offsets = [0_u64];

                device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);

                device.cmd_bind_index_buffer(command_buffer, geometry_manager.index_device_buffer_.buffer_handle_, 0, ash::vk::IndexType::UINT32);



                for vulkan_instance in frame_data.vulkan_instances_.iter()
                {
                    let mesh_location = geometry_manager.get_mesh_location(&vulkan_instance.mesh_id_);

                    device.cmd_bind_descriptor_sets(
                        command_buffer,
                        ash::vk::PipelineBindPoint::GRAPHICS,
                        self.pipeline_.unwrap().layout_.layout_handle_,
                        0,
                        &[vulkan_instance.descriptor_set_],
                        &[]
                    );
                    device.cmd_draw_indexed(
                        command_buffer,
                        mesh_location.index_count_ as u32,
                        1,
                        mesh_location.index_offset_ as u32,
                        mesh_location.vertex_offset_ as i32,
                        0,
                    );
                }
                //device.cmd_bind_descriptor_sets(command_buffer, ash::vk::PipelineBindPoint::GRAPHICS,)


                device.cmd_end_render_pass(command_buffer);
                device.end_command_buffer(command_buffer).expect("Could not end recording command buffer");
            }

        }
    }



/*
    fn record_draw_commands_forward(&mut self, device: &ash::Device, geometry_manager: &geometry_manager::GeometryManager)
    {
        let instances = self.instance_manager_.get_instances();

        for(i, &command_buffer) in self.command_dispatch_.command_buffers_.iter().enumerate()
        {
            let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo{
                s_type: ash::vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                flags: ash::vk::CommandBufferUsageFlags::SIMULTANEOUS_USE,
                p_inheritance_info: ptr::null()
            };

            unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info).expect("could not begin recording command buffer"); }

            let clear_values = [
                ash::vk::ClearValue{
                color: ash::vk::ClearColorValue{
                    float32: [0.0, 0.0, 0.0, 1.0],
                }},
            ash::vk::ClearValue{
                depth_stencil: ash::vk::ClearDepthStencilValue{
                    depth: 1.0,
                    stencil: 0,
                },
            }];

            let render_pass_begin_info = ash::vk::RenderPassBeginInfo{
                s_type: ash::vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: self.render_pass_.unwrap().handle_,
                framebuffer: self.swapchain_.swapchain_framebuffers_[i],
                render_area: ash::vk::Rect2D{
                    offset: ash::vk::Offset2D{x: 0, y: 0},
                    extent: self.swapchain_.swapchain_extent_
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };
            unsafe{
                device.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, ash::vk::SubpassContents::INLINE);

                let push_constant = 2u32;

                let bytes : [u8; 4] = push_constant.to_le_bytes();

                device.cmd_push_constants(
                    command_buffer,
                    self.pipeline_.unwrap().layout_.layout_handle_,
                    ash::vk::ShaderStageFlags::FRAGMENT,
                    0,
                    &bytes,
                );

                device.cmd_bind_pipeline(command_buffer, ash::vk::PipelineBindPoint::GRAPHICS, self.pipeline_.unwrap().pipeline_handle_);

                let vertex_buffers = [geometry_manager.vertex_device_buffer_.buffer_handle_];
                let offsets = [0_u64];

                device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);

                device.cmd_bind_index_buffer(command_buffer, geometry_manager.index_device_buffer_.buffer_handle_, 0, ash::vk::IndexType::UINT32);



                for (instance, ..) in instances.iter()
                {
                    let mesh_location = geometry_manager.get_mesh_location(&instance.mesh_id_);

                    device.cmd_bind_descriptor_sets(
                        command_buffer,
                        ash::vk::PipelineBindPoint::GRAPHICS,
                        self.pipeline_.unwrap().layout_.layout_handle_,
                        0,
                        &[instance.descriptor_set_],
                        &[]
                    );
                    device.cmd_draw_indexed(
                        command_buffer,
                        mesh_location.index_count_ as u32,
                        1,
                        mesh_location.index_offset_ as u32,
                        mesh_location.vertex_offset_ as i32,
                        0,
                    );
                }
                //device.cmd_bind_descriptor_sets(command_buffer, ash::vk::PipelineBindPoint::GRAPHICS,)


                device.cmd_end_render_pass(command_buffer);
                device.end_command_buffer(command_buffer).expect("Could not end recording command buffer");
            }

        }
    }
    */
}


