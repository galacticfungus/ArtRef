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
pub struct RasterizerSettings<'a, 'b: 'a> {
    settings: &'a vk::PipelineRasterizationStateCreateInfoBuilder<'b>,
}

pub struct ConfigureShaders<'a> {
    configured_shaders: Vec<ShaderData>,
    device: &'a erupt::DeviceLoader,
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
    shader_module: vk::ShaderModule,
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
    rasterizer_configuration: Option<vk::PipelineRasterizationStateCreateInfoBuilder<'a>>
}

pub struct InputAssembelySettings<'a,'b: 'a> {
    pipeline_assembely: &'a vk::PipelineInputAssemblyStateCreateInfoBuilder<'b>,
}

pub struct VertexBinding<'a> {
    binding: &'a vk::VertexInputBindingDescription,
    attributes: &'a mut Vec<vk::VertexInputAttributeDescription>,
}

pub enum AttributeFormat {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Double,
    UVec2,
    UVec3,
    UVec4,
    SVec2,
    SVec3,
    SVec4,
    DVec2,
    DVec3,
    DVec4,
}