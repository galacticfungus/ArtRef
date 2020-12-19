use super::{ConfigureShader, ConfigureShaders, ShaderData};
use erupt::vk1_0 as vk;
use std::convert::TryFrom;
use std::ffi::CString;

use crate::error::{Error, ErrorKind};

impl<'a> ConfigureShaders<'a> {
    pub fn new(device: &erupt::DeviceLoader) -> ConfigureShaders {
        ConfigureShaders {
            device,
            configured_shaders: Vec::new(),
        }
    }

    // TODO: Creating a shader module requires a configure object
    pub fn create_fragment_shader(&mut self, entry_name: &str, code: &[u32]) -> &mut Self
    {
        
        // self.shader_config_modules.push(shader_config);
        // let ConfigureShader {entry_name, shader_code, ..} = shader_config;
        let owned_name = CString::new(entry_name)
            .expect("Invalid UTF8 or at least a byte string containing a NUL byte");
        let shader_module_builder = vk::ShaderModuleCreateInfoBuilder::new();
        shader_module_builder.code(code);
        let shader_module = shader_module_builder.build();
        let vert_module = unsafe { self.device.create_shader_module(&shader_module, None, None) }
            .expect("Failed to create vertex shader module");
        let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
            .stage(vk::ShaderStageFlagBits::FRAGMENT)
            .name(owned_name.as_c_str())
            .module(vert_module)
            .build();
        self
    }

    pub fn create_vertex_shader(&mut self, entry_name: &str, code: &[u32]) -> &mut Self
    {
        let owned_name = CString::new(entry_name)
            .expect("Invalid UTF8 or at least a byte string containing a NUL byte");
        // TODO: is_valid can just return the internal structures
        // self.shader_config_modules.push(shader_config);
        // let ConfigureShader {entry_name, shader_code, ..} = shader_config;
        // TODO: Instead of using ConfigureShader as the type we store for when we create the pipeline, use ConfigureShader -> ShaderData and store that, avoid a lot of option checks
        let shader_module_builder = vk::ShaderModuleCreateInfoBuilder::new();
        shader_module_builder.code(code);
        let shader_module = shader_module_builder.build();
        let vert_module = unsafe { self.device.create_shader_module(&shader_module, None, None) }
            .expect("Failed to create vertex shader module");
        // TODO: Store the module, name and stage to create in a seperately for when we create the PipelineShaderStageCreateInfo
        let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
            .stage(vk::ShaderStageFlagBits::VERTEX)
            .name(owned_name.as_c_str())
            .module(vert_module)
            .build();
        self
    }
}

impl ShaderData {
    pub fn new(entry_name: &str, code: Vec<u32>, shader_type: vk::ShaderStageFlagBits) -> ShaderData {
        // TODO: Error Handling
        let owned_name = CString::new(entry_name)
            .expect("Invalid UTF8 or at least a byte string containing a NUL byte");
        ShaderData {
            entry_name: owned_name,
            shader_type,
            shader_code: code,
        }
    }
}

impl TryFrom<ConfigureShader> for ShaderData {
    type Error = crate::error::Error;
    fn try_from(shader: ConfigureShader) -> Result<Self, crate::Error> {
        // TODO: What even is this???
        let ConfigureShader {entry_name, shader_code, shader_type} = shader;
        match (entry_name, shader_code) {
            (Some(entry_name), Some(shader_code)) => {
                let shader_data = ShaderData {
                    entry_name,
                    shader_code,
                    shader_type: shader_type,
                };
                Ok(shader_data)
            }
            (entry_name, shader_code) => {
                let bad_entry = entry_name.is_none();
                let bad_code = shader_code.is_none();
                Err(Error::new(ErrorKind::InvalidPipelineConfig, None))
            }
        }
    }
}

// TODO: These functions can be exposed as a trait so that it isn't possible to call internal functions like is_valid

impl ConfigureShader {
    pub fn new(shader_type: vk::ShaderStageFlagBits) -> ConfigureShader {
        ConfigureShader {
            entry_name: None,
            shader_code: None,
            shader_type,
        }
    }

    pub fn shader_code(&mut self, code: Vec<u32>) -> &mut Self {
        self.shader_code = Some(code);
        self
    }

    pub fn entry_name(&mut self, entry_name: &str) -> &mut Self {
        let data = CString::new(entry_name)
            .expect("Invalid UTF8 or at least a byte string containing a NUL byte");
        self.entry_name = Some(data);
        self
    }
}
