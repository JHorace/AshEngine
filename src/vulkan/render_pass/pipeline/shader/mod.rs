use ash::version::DeviceV1_0;
use ash::vk::{
    PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateFlags, ShaderModuleCreateInfo,
    ShaderStageFlags, StructureType,
};
use ash::Device;

use std::ptr;

use crate::render_sequence::render_pass::pipeline::shader::{Shader, ShaderStage};
use std::ffi::CString;

pub struct VulkanShader {
    pub shader_module_: ShaderModule,
    pub shader_: Shader,
}

impl VulkanShader {
    pub fn new(device: &Device, shader: &Shader) -> VulkanShader {
        let shader_module_create_info = ShaderModuleCreateInfo {
            s_type: StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ShaderModuleCreateFlags::empty(),
            code_size: shader.bytes_.len(),
            p_code: shader.bytes_.as_ptr() as *const u32,
        };

        let shader_module = unsafe {
            device
                .create_shader_module(&shader_module_create_info, None)
                .expect("Could not create shader module")
        };

        VulkanShader {
            shader_module_: shader_module,
            shader_: shader.clone(),
        }
    }

    pub fn build_pipeline_shader_stage_create_info(&self) -> PipelineShaderStageCreateInfo {
        PipelineShaderStageCreateInfo {
            s_type: ash::vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineShaderStageCreateFlags::empty(),
            stage: match self.shader_.stage_ {
                ShaderStage::Vertex => ShaderStageFlags::VERTEX,
                ShaderStage::Fragment => ShaderStageFlags::FRAGMENT,
                ShaderStage::Compute => ShaderStageFlags::COMPUTE,
                ShaderStage::Geometry => ShaderStageFlags::GEOMETRY,
                _ => ShaderStageFlags::default(),
            },
            module: self.shader_module_,
            p_name: CString::new("main").unwrap().as_ptr(),
            p_specialization_info: ptr::null(),
        }
    }
}

/*
impl Shader
{
    pub unsafe fn from_spv(device: &ash::Device, path: &str, shader_type: ash::vk::ShaderStageFlags) -> Shader
    {
        Shader::from_bytes(device, Shader::read_spv_file(Path::new(path)), shader_type)
    }

    pub unsafe fn from_bytes(device: &ash::Device, bytes: Vec<u8>, shader_type: ash::vk::ShaderStageFlags) -> Shader
    {
        let shader_module_create_info = ash::vk::ShaderModuleCreateInfo{
            s_type: ash::vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::ShaderModuleCreateFlags::empty(),
            code_size: bytes.len(),
            p_code: bytes.as_ptr() as *const u32,
        };

        let shader_module_handle = device.create_shader_module(&shader_module_create_info, None).expect("Could not create shader module");
        Shader{ shader_module_handle_: shader_module_handle, shader_type_: shader_type, function_name_: CString::new("main").unwrap(), spirv_: bytes }
    }

    pub unsafe fn from_glsl(device: &ash::Device, path: &str, shader_type: ash::vk::ShaderStageFlags) -> Shader
    {
        let mut compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();

        let shader_kind = match shader_type{
            ash::vk::ShaderStageFlags::VERTEX => shaderc::ShaderKind::Vertex,
            ash::vk::ShaderStageFlags::FRAGMENT => shaderc::ShaderKind::Fragment,
            _ => shaderc::ShaderKind::InferFromSource,
        };

        let glsl = fs::read_to_string(path).unwrap();
        let bytes = compiler.compile_into_spirv(glsl.as_str(),
                                                shader_kind, path, "main", Some(&options)).unwrap().as_binary_u8().to_vec();

        Shader::from_bytes(device, bytes, shader_type)

    }

    pub fn build_pipeline_shader_stage_create_info(&self) -> ash::vk::PipelineShaderStageCreateInfo
    {
        ash::vk::PipelineShaderStageCreateInfo{
            s_type: ash::vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::PipelineShaderStageCreateFlags::empty(),
            stage: self.shader_type_,
            module: self.shader_module_handle_,
            p_name: self.function_name_.as_ptr(),
            p_specialization_info: ptr::null(),
        }
    }


    fn read_spv_file(path: &Path) -> Vec<u8>
    {
        let file = File::open(path).expect("Could not open shader file");

        file.bytes().filter_map(|byte| byte.ok()).collect()
    }

}
 */
