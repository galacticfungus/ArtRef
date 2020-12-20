use erupt::vk1_0 as vk;
use crate::error::{Error, ErrorKind};
use super::traits::ConfigureVertexInput;
use super::{ConfigureShaders, ConfigurePipeline};



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
        }
    }

    pub fn configure_shaders(&mut self, define_shaders: &mut dyn FnMut(&mut ConfigureShaders) -> Result<(), Error>) -> Result<&mut dyn ConfigureVertexInput, Error>
    {
        let mut configure_shaders = ConfigureShaders::new(self.device);
        if let Err(error) = define_shaders(&mut configure_shaders) {
            return Err(error);
        }
        self.configured_shaders = Some(configure_shaders);
        Ok(self)
    }

    // TODO: This function can fail if the bindings or attribute lengths exceed a u32, a device will support a fraction of this amount
    // pub fn configure_vertex_input<F>(mut self, configure_input: F) -> Self
    // where
    //     F: FnOnce(&mut ConfigureVertexInput) -> (),
    // {
        
    //     // let vert_input_stage = vk::PipelineVertexInputStateCreateInfoBuilder::new();
    //     // vert_input_stage.vertex_attribute_descriptions(vertex_attribute_descriptions)
    //     // vert_input_stage.vertex_binding_descriptions(vertex_binding_descriptions)
    //     // TODO: When configuring the input we should pass in the maximium allowed vertex inputs that the device allows
        
    //     let mut config = ConfigureVertexInput::new(&mut self.vertex_binding_descriptions, &mut self.vertex_attribute_descriptions);
    //     configure_input(&mut config);

    //     // We create a vk::PipelineVertexInputStateCreateInfo from the data this function creates
    //     // TODO: We cant create this struct now as references are destroyed when we move self
    //     // TODO: One option is to box the underlying datastructres that place that in self then valid references would remain valid as we only move a smart pointer
    //     // let b = vk::PipelineVertexInputStateCreateInfoBuilder::new()
    //     //     .vertex_attribute_descriptions(self.vertex_attribute_descriptions.as_slice())
    //     //     .vertex_binding_descriptions(self.vertex_binding_descriptions.as_slice());
    //     // self.vertex_input_info = Some(b);
    //     self
    // }

    // pub fn configure_input_assembely(
    //     mut self,
    //     topology: vk::PrimitiveTopology,
    //     enable_restart: bool,
    // ) -> Self {
    //     // TODO: Using this is dangerous, it sets the API to be non-updatable, while if we have an object that is passed to a user defined closure then using traits lets us expand on that objects functionality
    //     // VkPipelineInputAssemblyStateCreateInfo inputAssembly{};
    //     // inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
    //     // inputAssembly.primitiveRestartEnable = VK_FALSE;
    //     let pipeline_input = vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
    //         .topology(topology)
    //         .primitive_restart_enable(enable_restart);
    //     self.pipeline_input = Some(pipeline_input);
    //     self
    // }

    // pub fn configure_viewport<F>(mut self, create_viewport: F) -> Self
    // where
    //     F: Fn(&mut ViewportManager) -> (),
    // {
    //     // TODO: It's possible to create multiple viewports but its locked behind a gpu feature,
    //     // TODO: Each viewport has a scissor associated with it
    //     let mut mng = ViewportManager::new();
    //     create_viewport(&mut mng);

    //     // VkPipelineViewportStateCreateInfo viewportState{};
    //     // viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    //     // viewportState.viewportCount = 1;
    //     // viewportState.pViewports = &viewport;
    //     // viewportState.scissorCount = 1;
    //     // viewportState.pScissors = &scissor;
    //     // TODO: Verify a viewport was created ie if len is 0 then leave it as None?
    //     self.viewports_to_create = Some(mng.viewports);
    //     self
    // }

    // pub fn configure_rasterizer<F>(mut self, configure_rasterizer: F) -> Self where F: FnOnce(&mut ConfigureRasterizer)  {
    //     // Many of these options require the use of device features
    //     let mut config = ConfigureRasterizer::new();
    //     configure_rasterizer(&mut config);
        
    //     let builder = vk::PipelineRasterizationStateCreateInfoBuilder::new();
    //     // TODO: So a helper struct that can determine if certain options are available and allow customizing those options conditionally
    //     // TODO: Although you are more likely to create a different graphics pipe line altogether that modify one conditionally?
    //     config.build_rasterizer();
    //     builder.depth_bias_clamp(0.0);
    //     builder.depth_bias_enable(false);
    //     builder.polygon_mode(vk::PolygonMode::FILL);
    //     builder.cull_mode(vk::CullModeFlags::BACK);
    //     builder.rasterizer_discard_enable(false);
    //     builder.line_width(1.);
    //     self
    // }

    // pub fn configure_multisampling(mut self) -> Self {
    //     // TODO: Closure that allows specifying options but also passes in device limits to allow for configuration
    //     let b = vk::PipelineMultisampleStateCreateInfoBuilder::new();
    //     // VkPhysicalDeviceLimits
    //     b.rasterization_samples(vk::SampleCountFlagBits::_64);
    //     //         VkPipelineMultisampleStateCreateInfo multisampling{};
    //     // multisampling.sampleShadingEnable = VK_FALSE;
    //     // multisampling.rasterizationSamples = VK_SAMPLE_COUNT_1_BIT;
    //     // multisampling.minSampleShading = 1.0f; // Optional
    //     // multisampling.pSampleMask = nullptr; // Optional
    //     // multisampling.alphaToCoverageEnable = VK_FALSE; // Optional
    //     // multisampling.alphaToOneEnable = VK_FALSE; // Optional
    //     self
    // }

    // pub fn configure_depth_stencil_tests(mut self) -> Self {
    //     // VkPipelineDepthStencilStateCreateInfo
    //     // TODO: Much like configure multisampling
    //     let b = vk::PipelineDepthStencilStateCreateInfoBuilder::new();
    //     self
    // }

    // pub fn configure_color_blend(&self) -> &Self {
    //     // VkPipelineColorBlendAttachmentState colorBlendAttachment{};
    //     // colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
    //     // colorBlendAttachment.blendEnable = VK_FALSE;
    //     // colorBlendAttachment.srcColorBlendFactor = VK_BLEND_FACTOR_ONE; // Optional
    //     // colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ZERO; // Optional
    //     // colorBlendAttachment.colorBlendOp = VK_BLEND_OP_ADD; // Optional
    //     // colorBlendAttachment.srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE; // Optional
    //     // colorBlendAttachment.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO; // Optional
    //     // colorBlendAttachment.alphaBlendOp = VK_BLEND_OP_ADD; // Optional
    //     self
    // }

    // pub fn configure_dynamic_state(mut self) -> Self {
    //     // TODO: closure that passes in a dynamic state object where you can request certain dynamic states
    //     //         VkDynamicState dynamicStates[] = {
    //     //     VK_DYNAMIC_STATE_VIEWPORT,
    //     //     VK_DYNAMIC_STATE_LINE_WIDTH
    //     // };
    //     let b = vk::PipelineDynamicStateCreateInfoBuilder::new();
    //     // VkPipelineDynamicStateCreateInfo dynamicState{};
    //     // dynamicState.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    //     // dynamicState.dynamicStateCount = 2;
    //     // dynamicState.pDynamicStates = dynamicStates;
    //     self
    // }

    // pub fn configure_layout(mut self) -> Self {
    //     // TODO: Create the uniform variables - ie globals that are passed to the shaders

    //     // VkPipelineLayoutCreateInfo pipelineLayoutInfo{};
    //     // pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    //     // pipelineLayoutInfo.setLayoutCount = 0; // Optional
    //     // pipelineLayoutInfo.pSetLayouts = nullptr; // Optional
    //     // pipelineLayoutInfo.pushConstantRangeCount = 0; // Optional
    //     // pipelineLayoutInfo.pPushConstantRanges = nullptr; // Optional

    //     // if (vkCreatePipelineLayout(device, &pipelineLayoutInfo, nullptr, &pipelineLayout) != VK_SUCCESS) {
    //     //  throw std::runtime_error("failed to create pipeline layout!");
    //     // }
    //     self
    // }

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
