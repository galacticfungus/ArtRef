use super::{ConfigureShaders, ShaderData};
use erupt::vk1_0 as vk;
use std::ffi::CString;

use crate::error::{Error, ErrorKind};

impl<'a> ConfigureShaders<'a> {
    pub fn new(device: &erupt::DeviceLoader) -> ConfigureShaders {
        ConfigureShaders {
            device,
            configured_shaders: Vec::new(),
        }
    }

    pub fn create_fragment_shader(&mut self, entry_name: &str, code: &[u32]) -> Result<&mut Self, Error>
    {
        let shader_data = self.prepare_shader(entry_name, code, vk::ShaderStageFlagBits::FRAGMENT)?;
        // let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
        //     .stage(vk::ShaderStageFlagBits::FRAGMENT)
        //     .name(owned_name.as_c_str())
        //     .module(vert_module)
        //     .build();
        self.configured_shaders.push(shader_data);
        Ok(self)
    }

    // Creates the data associated with a shader so that it can be easily included when the pipeline is ready to be created.
    // The shader module is the only data structure that is created before the pipeline is created.
    pub fn prepare_shader(&self, entry_name: &str, code: &[u32], shader_type: vk::ShaderStageFlagBits) -> Result<ShaderData, Error> {
        let owned_name = CString::new(entry_name)
            .map_err(|err| Error::new(ErrorKind::InvalidShaderEntryMethodName(Vec::from(entry_name.as_bytes())), None))?;
        let shader_module_builder = vk::ShaderModuleCreateInfoBuilder::new();
        shader_module_builder.code(code);
        let shader_module = unsafe { self.device.create_shader_module(&shader_module_builder, None, None) }
            .expect("Failed to create vertex shader module");
        let shader_data = ShaderData::new(owned_name, shader_module, shader_type);
        return Ok(shader_data);
    }
    // Create a vertex shader with a given entry name and code
    pub fn create_vertex_shader(&mut self, entry_name: &str, code: &[u32]) -> Result<&mut Self, Error>
    {
        let shader_data = self.prepare_shader(entry_name, code, vk::ShaderStageFlagBits::VERTEX)?;
        // let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
        //     .stage(vk::ShaderStageFlagBits::FRAGMENT)
        //     .name(owned_name.as_c_str())
        //     .module(vert_module)
        //     .build();
        self.configured_shaders.push(shader_data);
        Ok(self)
    }
}

impl ShaderData {
    pub fn new(entry_name: CString, module: vk::ShaderModule, shader_type: vk::ShaderStageFlagBits) -> ShaderData {
        ShaderData {
            entry_name,
            shader_type,
            shader_module: module,
        }
    }
}
