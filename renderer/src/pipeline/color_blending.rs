use crate::ConfigurePipeline;
use super::{ColorBlendingSettings, traits::ConfigureColorBlending};
use erupt::vk1_0 as vk;
impl<'a> ConfigureColorBlending for ConfigurePipeline<'a> {
    fn configure_blending(&mut self, configure_blending: &mut dyn FnMut(&mut ColorBlendingSettings)) {
        let mut blend_config = vk::PipelineColorBlendStateCreateInfoBuilder::new();
        let mut settings = ColorBlendingSettings::new(&mut blend_config);
        configure_blending(&mut settings);
    }
}

impl<'a, 'b: 'a> ColorBlendingSettings<'a, 'b> {
    pub fn new(blend_settings: &'a mut vk::PipelineColorBlendStateCreateInfoBuilder<'b>) -> ColorBlendingSettings<'a, 'b> {
        ColorBlendingSettings {
            settings: blend_settings,
        }
    }

    pub fn enable_blend(&mut self, enable_blend: bool) {
        // This setting needs access to the framebuffer image attachments
        // Do we create them or do we accept a list to use
        // TODO: Allow creation of framebuffer image attachments or accept a list of framebuffer image attachments
        //self.settings.logic_op_enable(logic_op_enable);
    }
}

// pub fn configure_color_blend(&self) -> &Self {
        // VkPipelineColorBlendAttachmentState colorBlendAttachment{};
        // colorBlendAttachment.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
        // colorBlendAttachment.blendEnable = VK_FALSE;
        // colorBlendAttachment.srcColorBlendFactor = VK_BLEND_FACTOR_ONE; // Optional
        // colorBlendAttachment.dstColorBlendFactor = VK_BLEND_FACTOR_ZERO; // Optional
        // colorBlendAttachment.colorBlendOp = VK_BLEND_OP_ADD; // Optional
        // colorBlendAttachment.srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE; // Optional
        // colorBlendAttachment.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO; // Optional
        // colorBlendAttachment.alphaBlendOp = VK_BLEND_OP_ADD; // Optional
    //     self
    // }