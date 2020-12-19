mod traits;
mod input_assembely;
mod rasterizer;
mod shaders;
mod vertex_input;
mod viewport;
mod config;

use erupt::vk1_0 as vk;

pub use std::ffi::CString;

/// Configure the rasterization options
#[derive(Debug, Clone)]
pub struct RasterizerSettings {
    depth_clamp: Option<bool>,
    rasterizer_discard: Option<bool>,
    polygon_mode: Option<vk::PolygonMode>,
    line_width: Option<f32>,
    cull_mode: Option<vk::CullModeFlags>,
    front_face: Option<vk::FrontFace>,
    depth_bias: Option<bool>,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
}

pub struct ConfigureShaders<'a> {
    configured_shaders: Vec<ConfigureShader>,
    device: &'a erupt::DeviceLoader,
}

pub struct ConfigureShader {
    entry_name: Option<CString>,
    shader_code: Option<Vec<u32>>,
    shader_type: vk::ShaderStageFlagBits
}

pub struct VertexInputSettings<'a> {
    vertex_attributes: &'a mut Vec<vk::VertexInputAttributeDescription>,
    vertex_bindings: &'a mut Vec<vk::VertexInputBindingDescription>,
}
pub struct ViewportManager {
    viewports: Vec<Viewport>,
}
pub struct Viewport {
    viewport: vk::Viewport,
    scissor: vk::Rect2D,
}
pub struct ShaderData {
    entry_name: CString,
    shader_code: Vec<u32>,
    shader_type: vk::ShaderStageFlagBits,
}

pub struct ConfigurePipeline<'a> {
    device: &'a erupt::DeviceLoader,
    shader_config_modules: Vec<vk::ShaderModule>,
    pipeline_input: Option<vk::PipelineInputAssemblyStateCreateInfoBuilder<'a>>,
    viewports_to_create: Option<Vec<Viewport>>,
    vertex_attribute_descriptions: Vec<vk::VertexInputAttributeDescription>,
    vertex_binding_descriptions: Vec<vk::VertexInputBindingDescription>,
    vertex_input_info: Option<vk::PipelineVertexInputStateCreateInfo>,
    configured_shaders: Option<ConfigureShaders<'a>>,
}