use super::traits::{ConfigureInputAssembely, ConfigureViewport};
use super::ConfigurePipeline;
use erupt::vk1_0 as vk;

impl<'a> ConfigureInputAssembely for ConfigurePipeline<'a> {
    fn configure_input_assembely(
        &mut self,
        topology: vk::PrimitiveTopology,
        enable_restart: bool,
    ) -> &mut dyn ConfigureViewport {
        // TODO: Using this is dangerous, it sets the API to be non-updatable, while if we have an object that is passed to a user defined closure then using traits lets us expand on that objects functionality
        // VkPipelineInputAssemblyStateCreateInfo inputAssembly{};
        // inputAssembly.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
        // inputAssembly.primitiveRestartEnable = VK_FALSE;
        let pipeline_input = vk::PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(topology)
            .primitive_restart_enable(enable_restart);
        self.pipeline_input = Some(pipeline_input);
        self
    }
}
