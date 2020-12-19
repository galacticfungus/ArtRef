use super::traits::{ConfigureInputAssembely, ConfigureVertexInput};
use super::ConfigurePipeline;
use super::VertexInputSettings;

use erupt::vk1_0 as vk;

impl<'a> ConfigureVertexInput for ConfigurePipeline<'a> {
    fn configure_vertex_input(
        &mut self,
        configure_input: &mut dyn FnMut(&mut VertexInputSettings) -> (),
    ) -> &mut dyn ConfigureInputAssembely {
        // let vert_input_stage = vk::PipelineVertexInputStateCreateInfoBuilder::new();
        // vert_input_stage.vertex_attribute_descriptions(vertex_attribute_descriptions)
        // vert_input_stage.vertex_binding_descriptions(vertex_binding_descriptions)
        // TODO: When configuring the input we should pass in the maximium allowed vertex inputs that the device allows

        let mut config = VertexInputSettings::new(
            &mut self.vertex_binding_descriptions,
            &mut self.vertex_attribute_descriptions,
        );
        configure_input(&mut config);

        // We create a vk::PipelineVertexInputStateCreateInfo from the data this function creates
        // TODO: We cant create this struct now as references are destroyed when we move self
        // TODO: One option is to box the underlying datastructres that place that in self then valid references would remain valid as we only move a smart pointer
        // let b = vk::PipelineVertexInputStateCreateInfoBuilder::new()
        //     .vertex_attribute_descriptions(self.vertex_attribute_descriptions.as_slice())
        //     .vertex_binding_descriptions(self.vertex_binding_descriptions.as_slice());
        // self.vertex_input_info = Some(b);
        self
    }
}

impl<'a> VertexInputSettings<'a> {
    pub fn new(
        input_bindings: &'a mut Vec<vk::VertexInputBindingDescription>,
        input_attributes: &'a mut Vec<vk::VertexInputAttributeDescription>,
    ) -> VertexInputSettings<'a> {
        // let b = vk::PipelineVertexInputStateCreateInfoBuilder::new();
        // vk::VertexInputAttributeDescriptionBuilder::new();
        // b.vertex_attribute_descriptions(vertex_attribute_descriptions)
        // b.vertex_binding_descriptions(vertex_binding_descriptions)
        // TODO: Instead of creating the vectors here we instead pass mutable references to the containing structure, thus avoiding moving the data out of this structure when we need it later
        // b.flags()

        VertexInputSettings {
            vertex_bindings: input_bindings,
            vertex_attributes: input_attributes,
        }
    }

    pub fn add_binding(
        &mut self,
        binding: u32,
        input_rate: vk::VertexInputRate,
        stride: u32,
    ) -> &mut Self {
        let binding = vk::VertexInputBindingDescriptionBuilder::new()
            .binding(binding)
            .input_rate(input_rate)
            .stride(stride)
            .build();
        self.vertex_bindings.push(binding);
        self
    }

    pub fn add_attribute(
        &mut self,
        binding: u32,
        format: vk::Format,
        location: u32,
        offset: u32,
    ) -> &mut Self {
        let attribute = vk::VertexInputAttributeDescriptionBuilder::new()
            .binding(binding)
            .format(format)
            .offset(offset)
            .location(location)
            .build();
        self.vertex_attributes.push(attribute);
        self
    }
}
