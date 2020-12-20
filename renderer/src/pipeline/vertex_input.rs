use crate::Feature;

use super::traits::{ConfigureInputAssembely, ConfigureVertexInput};
use super::ConfigurePipeline;
use super::VertexInputSettings;

use erupt::vk1_0 as vk;
use erupt::vk1_0::Format as Format;

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

impl AttributeFormat {

}

// TODO: Use rusts types

impl From<AttributeFormat> for vk::Format {
    fn from(format: AttributeFormat) -> Self {
        match format {
            AttributeFormat::Double => Format::R64_SFLOAT,
            AttributeFormat::Float => Format::R32_SFLOAT,
            AttributeFormat::Vec2 => Format::R32G32_SFLOAT,
            AttributeFormat::Vec3 => Format::R32G32B32_SFLOAT,
            AttributeFormat::Vec4 => Format::R32G32B32A32_SFLOAT,
            AttributeFormat::UVec2 => Format::R32G32_UINT,
            AttributeFormat::UVec3 => Format::R32G32B32_UINT,
            AttributeFormat::UVec4 => Format::R32G32B32A32_UINT,
            AttributeFormat::SVec2 => Format::R32G32_SINT,
            AttributeFormat::SVec3 => Format::R32G32B32_SINT,
            AttributeFormat::SVec4 => Format::R32G32B32A32_SINT,
            AttributeFormat::DVec2 => Format::R64G64_SFLOAT,
            AttributeFormat::DVec3 => Format::R64G64B64_SFLOAT,
            AttributeFormat::DVec4 => Format::R64G64B64A64_SFLOAT,
        }
    }
}

pub struct VertexBinding<'a> {
    binding: &'a vk::VertexInputBindingDescription,
    attributes: &'a mut Vec<vk::VertexInputAttributeDescription>,
}

impl<'a> VertexBinding<'a> {
    pub fn new(binding: &'a vk::VertexInputBindingDescription, attributes: &'a mut Vec<vk::VertexInputAttributeDescription>) -> VertexBinding<'a> {
        VertexBinding {
            binding,
            attributes,
        }
    }
    pub fn add_attribute(&mut self, location: u32, offset: u32, format: AttributeFormat) -> &mut Self {
        // binding
        let builder = vk::VertexInputAttributeDescriptionBuilder::new();
        builder.binding(self.binding.binding);
        // The location in the shader
        builder.location(location);
        // type of attribute
        builder.format(format.into());
        // offset
        builder.offset(offset);
        self.attributes.push(*builder);
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
    ) -> VertexBinding {
        let binding = vk::VertexInputBindingDescriptionBuilder::new()
            // For multiple bindings
            .binding(binding)
            // Per vertex or per instance binding
            .input_rate(input_rate)
            // Specifies the number of bytes per vertex
            .stride(stride)
            .build();
        self.vertex_bindings.push(binding);
        VertexBinding::new(&self.vertex_bindings[self.vertex_bindings.len() - 1], &mut self.vertex_attributes)
    }
}
