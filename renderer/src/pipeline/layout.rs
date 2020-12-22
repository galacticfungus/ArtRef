use crate::ConfigurePipeline;

use super::{LayoutSettings, traits::ConfigureLayout};
use erupt::vk1_0 as vk;
// pub fn configure_layout(mut self) -> Self {
// TODO: Create the uniform variables - ie globals that are passed to the shaders

// VkPipelineLayoutCreateInfo pipelineLayoutInfo{};
// pipelineLayoutInfo.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
// pipelineLayoutInfo.setLayoutCount = 0; // Optional
// pipelineLayoutInfo.pSetLayouts = nullptr; // Optional
// pipelineLayoutInfo.pushConstantRangeCount = 0; // Optional
// pipelineLayoutInfo.pPushConstantRanges = nullptr; // Optional

// if (vkCreatePipelineLayout(device, &pipelineLayoutInfo, nullptr, &pipelineLayout) != VK_SUCCESS) {
//  throw std::runtime_error("failed to create pipeline layout!");
// }
//     self
// }

impl<'a> ConfigureLayout for ConfigurePipeline<'a> {
    fn configure_layout(&mut self, configure_layout: &mut dyn FnMut(&mut super::LayoutSettings)) {
        let mut pipeline_layout_settings = vk::PipelineLayoutCreateInfoBuilder::new();
        let mut push_constants: Vec<vk::PushConstantRangeBuilder> = Vec::new();
        let mut layout_settings = LayoutSettings::new(&mut pipeline_layout_settings, &mut push_constants);
        // TODO: Should return a ConfiguredPipeline, ie configured but not created
        // TODO: Creating a pipeline should not free a configured pipeline
    }
}

impl<'a, 'b: 'a> LayoutSettings<'a, 'b> {
    pub fn new(pipeline_settings: &'a mut vk::PipelineLayoutCreateInfoBuilder<'b>, push_constants: &'a mut Vec<vk::PushConstantRangeBuilder<'b>>) -> LayoutSettings<'a, 'b> {
        LayoutSettings {
            pipeline_settings,
            push_constants,
        }
    }

    pub fn add_descriptor_set_layout(&mut self) {
        // Uniform buffers etc - this is tricky because they fall outside the scope of VulkanFlex???
        // VulkanFlex doesn't need to create the descriptor sets it just needs to access them
        // self.pipeline_settings.set_layouts(push_constant_ranges)
    }

    /// Push constants represent a high speed path to modify constant data in pipelines that is expected to outperform memory-backed resource updates
    pub fn add_push_constant(&mut self, push_constant: vk::PushConstantRangeBuilder<'b>) {

        self.push_constants.push(push_constant);
    }
}