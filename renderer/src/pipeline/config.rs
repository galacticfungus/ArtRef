use super::traits::ConfigureVertexInput;
use super::{ConfigurePipeline, ConfigureShaders};
use crate::error::{Error, ErrorKind};
use erupt::vk1_0 as vk;

impl<'a> ConfigurePipeline<'a> {
    pub fn new(device: &erupt::DeviceLoader) -> ConfigurePipeline {
        ConfigurePipeline {
            device,
            shader_config_modules: Vec::new(),
            pipeline_input: None,
            viewports_to_create: None,
            vertex_attribute_descriptions: Vec::new(),
            vertex_binding_descriptions: Vec::new(),
            vertex_input_info: None,
            configured_shaders: None,
            rasterizer_configuration: None,
            multisample_config: None,
            sample_masks: Vec::new(),
            color_blending: None,
        }
    }

    pub fn configure_shaders(
        &mut self,
        define_shaders: &mut dyn FnMut(&mut ConfigureShaders) -> Result<(), Error>,
    ) -> Result<&mut dyn ConfigureVertexInput, Error> {
        let mut configure_shaders = ConfigureShaders::new(self.device);
        if let Err(error) = define_shaders(&mut configure_shaders) {
            return Err(error);
        }
        self.configured_shaders = Some(configure_shaders);
        Ok(self)
    }

    

    

    // pub fn create_pipeline(mut self) -> Self {
    //     // TODO: Takes in a render pass as a reference and returns a pipeline object
    //     //self.device.create_graphics_pipelines(pipeline_cache, create_infos, allocator)
    //     self
    // }
}

// VkPipelineVertexInputStateCreateInfo vertexInputInfo{};
// vertexInputInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
// vertexInputInfo.vertexBindingDescriptionCount = 0;
// vertexInputInfo.pVertexBindingDescriptions = nullptr; // Optional
// vertexInputInfo.vertexAttributeDescriptionCount = 0;
// vertexInputInfo.pVertexAttributeDescriptions = nullptr; // Optional

// TODO: These configure types some strong bindings to the parent data structure, is this code smell or ok?

// impl<'a> ConfigureVertexInput<'a> {
//     pub fn new(
//         input_bindings: &'a mut Vec<vk::VertexInputBindingDescription>,
//         input_attributes: &'a mut Vec<vk::VertexInputAttributeDescription>,
//     ) -> ConfigureVertexInput<'a> {
//         // let b = vk::PipelineVertexInputStateCreateInfoBuilder::new();
//         // vk::VertexInputAttributeDescriptionBuilder::new();
//         // b.vertex_attribute_descriptions(vertex_attribute_descriptions)
//         // b.vertex_binding_descriptions(vertex_binding_descriptions)
//         // TODO: Instead of creating the vectors here we instead pass mutable references to the containing structure, thus avoiding moving the data out of this structure when we need it later
//         // b.flags()

//         ConfigureVertexInput {
//             vertex_bindings: input_bindings,
//             vertex_attributes: input_attributes,
//         }
//     }

//     pub fn add_binding(&mut self, binding: u32, input_rate:vk::VertexInputRate, stride: u32) -> &mut Self {
//         let binding = vk::VertexInputBindingDescriptionBuilder::new()
//             .binding(binding).input_rate(input_rate).stride(stride).build();
//         self.vertex_bindings.push(binding);
//         self
//     }

//     pub fn add_attribute(&mut self, binding: u32, format: vk::Format, location: u32, offset: u32) -> &mut Self {
//         let attribute = vk::VertexInputAttributeDescriptionBuilder::new()
//             .binding(binding)
//             .format(format)
//             .offset(offset)
//             .location(location)
//             .build();
//         self.vertex_attributes.push(attribute);
//         self
//     }
// }

// impl<'a> ConfigureShaders<'a> {

//     pub fn new(device: &erupt::DeviceLoader) -> ConfigureShaders {
//         ConfigureShaders {
//             device,
//             configured_shaders: Vec::new(),
//         }
//     }

//     // TODO: Creating a shader module requires a configure object
//     pub fn create_fragment_shader<F>(&mut self, define_shader: F) -> &mut Self
//     where
//         F: FnOnce(&mut ConfigureShader) -> (),
//     {
//         let mut shader_config = ConfigureShader::new(vk::ShaderStageFlagBits::FRAGMENT);
//         define_shader(&mut shader_config);
//         // self.shader_config_modules.push(shader_config);
//         // let ConfigureShader {entry_name, shader_code, ..} = shader_config;
//         let shader_data = ShaderData::try_from(shader_config).expect("Shader was invalid");
//         let shader_module_builder = vk::ShaderModuleCreateInfoBuilder::new();
//                 shader_module_builder.code(shader_data.shader_code.as_slice());
//                 let shader_module = shader_module_builder.build();
//                 let vert_module =
//                     unsafe { self.device.create_shader_module(&shader_module, None, None) }
//                         .expect("Failed to create vertex shader module");
//                 let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
//                     .stage(vk::ShaderStageFlagBits::VERTEX)
//                     .name(shader_data.entry_name.as_c_str())
//                     .module(vert_module)
//                     .build();
//         self
//     }

//     pub fn create_vertex_shader<F>(&mut self, define_shader: F) -> &mut Self
//     where
//         F: FnOnce(&mut ConfigureShader) -> (),
//     {
//         let mut shader_config = ConfigureShader::new(vk::ShaderStageFlagBits::VERTEX);
//         define_shader(&mut shader_config);
//         let shader_data = ShaderData::try_from(shader_config).expect("Shader was invalid");
//         // TODO: is_valid can just return the internal structures
//         // self.shader_config_modules.push(shader_config);
//         // let ConfigureShader {entry_name, shader_code, ..} = shader_config;
//         // TODO: Instead of using ConfigureShader as the type we store for when we create the pipeline, use ConfigureShader -> ShaderData and store that, avoid a lot of option checks
//         let shader_module_builder = vk::ShaderModuleCreateInfoBuilder::new();
//                 shader_module_builder.code(shader_data.shader_code.as_slice());
//                 let shader_module = shader_module_builder.build();
//                 let vert_module =
//                     unsafe { self.device.create_shader_module(&shader_module, None, None) }
//                         .expect("Failed to create vertex shader module");
//                 let stage_info = vk::PipelineShaderStageCreateInfoBuilder::new()
//                     .stage(vk::ShaderStageFlagBits::VERTEX)
//                     .name(shader_data.entry_name.as_c_str())
//                     .module(vert_module)
//                     .build();
//         self
//         // vertShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
//         // vertShaderStageInfo.stage = VK_SHADER_STAGE_VERTEX_BIT;
//         // vertShaderStageInfo.module = vertShaderModule;
//         // vertShaderStageInfo.pName = "main";

//         // VkPipelineShaderStageCreateInfo fragShaderStageInfo{};
//         // fragShaderStageInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
//         // fragShaderStageInfo.stage = VK_SHADER_STAGE_FRAGMENT_BIT;
//         // fragShaderStageInfo.module = fragShaderModule;
//         // fragShaderStageInfo.pName = "main";

//         // VkPipelineShaderStageCreateInfo shaderStages[] = {vertShaderStageInfo, fragShaderStageInfo};
//     }
// }

// impl ConfigureRasterizer {
//     pub fn polygon_mode(&mut self, polygon_mode: vk::PolygonMode) {
//         self.polygon_mode = Some(polygon_mode);
//     }

//     pub fn depth_clamp(&mut self, depth_clamp: bool) {
//         self.depth_clamp = Some(depth_clamp);
//     }

//     pub fn rasterizer_discard(&mut self, rasterizer_discard: bool) {
//         self.rasterizer_discard = Some(rasterizer_discard);
//     }

//     pub fn line_width(&mut self, line_width: f32) {
//         self.line_width = Some(line_width);
//     }

//     pub fn cull_mode(&mut self, cull_mode: vk::CullModeFlags) {
//         self.cull_mode = Some(cull_mode);
//     }

//     pub fn front_face(&mut self, front_face: vk::FrontFace) {
//         self.front_face = Some(front_face);
//     }

//     pub fn depth_bias(&mut self, depth_bias: bool) {
//         self.depth_bias = Some(depth_bias);
//     }

//     pub fn depth_bias_constant_factor(&mut self, depth_bias_constant_factor: f32) {
//         self.depth_bias_constant_factor = depth_bias_constant_factor;
//     }

//     pub fn depth_bias_clamp(&mut self, depth_bias_clamp: f32) {
//         self.depth_bias_clamp = depth_bias_clamp;
//     }

//     pub fn depth_bias_slope_factor(&mut self, depth_bias_slope_factor: f32) {
//         self.depth_bias_slope_factor = depth_bias_slope_factor;
//     }

//     pub fn new() -> ConfigureRasterizer {
//         ConfigureRasterizer {
//             polygon_mode: None,
//             rasterizer_discard: None,
//             depth_clamp: None,
//             line_width: None,
//             cull_mode: None,
//             front_face: None,
//             depth_bias: None,
//             depth_bias_constant_factor: 0.0,
//             depth_bias_clamp: 0.0,
//             depth_bias_slope_factor: 0.0,
//         }
//     }

//     pub fn build_rasterizer(self) -> Result<vk::PipelineRasterizationStateCreateInfo, Error> {
//         // let ConfigureRasterizer {cull_mode, depth_bias, depth_bias_clamp, depth_bias_constant_factor, depth_bias_slope_factor, depth_clamp, front_face, line_width, polygon_mode, rasterizer_discard} = self;
//         // TODO: Unwrap all the fields or return None?
//         let config = self;
//         let cull_mode = config.cull_mode.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
//         let depth_clamp = config.depth_clamp.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
//         // let depth_bias_constant_factor = self.depth_bias_constant_factor.ok_or(Error::InvalidRasterizationConfig("Depth Clamp was not set during the fixed function rasterization stage", self))?;
//         let rasterizer = vk::PipelineRasterizationStateCreateInfoBuilder::new()
//             .cull_mode(cull_mode)
//             .depth_clamp_enable(depth_clamp)
//             .depth_bias_constant_factor(config.depth_bias_constant_factor)
//             .build();
//         Ok(rasterizer)
//     }
// }

// impl ViewportManager {
//     pub fn new() -> ViewportManager {
//         ViewportManager {
//             viewports: Vec::new(),
//         }
//     }
//     pub fn create_viewport(
//         &mut self,
//         x: f32,
//         y: f32,
//         width: f32,
//         height: f32,
//         min_depth: f32,
//         max_depth: f32,
//         scissor_x: i32,
//         scissor_y: i32,
//         scissor_width: u32,
//         scissor_height: u32,
//     ) {
//         // TODO: Check that mutiple viewports are supported
//         // Width and Height are limited to the max values of a framebuffer
//         let viewport = Viewport::new(
//             x,
//             y,
//             width,
//             height,
//             min_depth,
//             max_depth,
//             scissor_x,
//             scissor_y,
//             scissor_width,
//             scissor_height,
//         );
//         self.viewports.push(viewport);
//     }
// }

// impl Viewport {
//     pub fn new(
//         x: f32,
//         y: f32,
//         width: f32,
//         height: f32,
//         min_depth: f32,
//         max_depth: f32,
//         scissor_x: i32,
//         scissor_y: i32,
//         scissor_width: u32,
//         scissor_height: u32,
//     ) -> Viewport {
//         Viewport {
//             viewport: vk::ViewportBuilder::new()
//                 .height(height)
//                 .width(width)
//                 .x(x)
//                 .y(y)
//                 .build(),
//             scissor: vk::Rect2DBuilder::new()
//                 .extent(
//                     vk::Extent2DBuilder::new()
//                         .height(scissor_height)
//                         .width(scissor_width)
//                         .build(),
//                 )
//                 .offset(vk::Offset2DBuilder::new().x(scissor_x).y(scissor_y).build())
//                 .build(),
//         }
//     }
// }

// use std::convert::TryFrom;

// impl TryFrom<ConfigureShader> for ShaderData {
//     type Error = crate::error::Error;
//     fn try_from(shader: ConfigureShader) -> Result<Self, crate::Error> {
//         match (shader.entry_name, shader.shader_code) {
//             (Some(entry_name), Some(shader_code)) => {

//                 let shader_data = ShaderData {
//                     entry_name,
//                     shader_code,
//                     shader_type: shader.shader_type,
//                 };
//                 Ok(shader_data)
//             },
//             (entry_name, shader_code) => {
//                 let bad_entry = entry_name.is_none();
//                 let bad_code = shader_code.is_none();
//                 Err(Error::new(ErrorKind::InvalidPipelineConfig, None))

//             },
//         }
//     }
// }

// // TODO: These functions can be exposed as a trait so that it isn't possible to call internal functions like is_valid

// impl ConfigureShader {
//     pub fn new(shader_type: vk::ShaderStageFlagBits) -> ConfigureShader {
//         ConfigureShader {
//             entry_name: None,
//             shader_code: None,
//             shader_type,
//         }
//     }

//     pub fn shader_code(&mut self, code: Vec<u32>) -> &mut Self {
//         self.shader_code = Some(code);
//         self
//     }

//     pub fn entry_name(&mut self, entry_name: &str) -> &mut Self {
//         let data = CString::new(entry_name)
//             .expect("Invalid UTF8 or at least a bye string containing a NUL byte");
//         self.entry_name = Some(data);
//         self
//     }
// }
