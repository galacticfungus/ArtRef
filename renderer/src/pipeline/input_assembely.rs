use super::traits::{ConfigureInputAssembely, ConfigureViewport};
use super::{ConfigurePipeline, InputAssembelySettings};
use erupt::vk1_0 as vk;

impl<'a> ConfigureInputAssembely for ConfigurePipeline<'a> {
    fn configure_input_assembely(
        &mut self,
        configure_assembely: &mut dyn FnMut(&mut InputAssembelySettings),
    ) -> &mut dyn ConfigureViewport {
        // TODO: Using this is dangerous, it sets the API to be non-updatable, while if we have an object that is passed to a user defined closure then using traits lets us expand on that objects functionality
        // VkPipelineInputAssemblyStateCreateInfo inputAssembly{};
        // inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
        // inputAssembly.primitiveRestartEnable = VK_FALSE;
        let mut pipeline_input = vk::PipelineInputAssemblyStateCreateInfoBuilder::new();
        let mut input_settings = InputAssembelySettings::new(&mut pipeline_input);
        configure_assembely(&mut input_settings);
        self.pipeline_input = Some(pipeline_input);
        self
    }
}

impl<'a, 'b: 'a> InputAssembelySettings<'a, 'b> {
    pub fn new(
        pipeline_input: &'a mut vk::PipelineInputAssemblyStateCreateInfoBuilder<'b>,
    ) -> InputAssembelySettings<'a, 'b> {
        InputAssembelySettings {
            pipeline_assembely: pipeline_input,
        }
    }

    pub fn set_topology(&mut self, topology_to_use: vk::PrimitiveTopology) {
        self.pipeline_assembely.topology(topology_to_use);
    }

    pub fn set_restart(&mut self, can_restart: bool) {
        self.pipeline_assembely
            .primitive_restart_enable(can_restart);
    }
}
