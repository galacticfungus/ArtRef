mod config;
use erupt::vk1_0 as vk;
pub use config::ConfigurePipeline;
pub use std::ffi::CString;

/// Configure the rasterization options
#[derive(Debug, Clone)]
pub struct ConfigureRasterizer {
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