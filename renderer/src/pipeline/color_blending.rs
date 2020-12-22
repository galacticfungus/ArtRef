use super::{traits::ConfigureColorBlending, ColorBlendingSettings, ColorBlendingType};
use crate::ConfigurePipeline;
use erupt::vk1_0 as vk;
impl<'a> ConfigureColorBlending for ConfigurePipeline<'a> {
    fn configure_blending(
        &mut self,
        configure_blending: &mut dyn FnMut(&mut ColorBlendingSettings),
    ) {
        let mut pipeline_blend_settings = vk::PipelineColorBlendStateCreateInfoBuilder::new();
        let mut blending_settings = ColorBlendingSettings::new(&mut pipeline_blend_settings);

        configure_blending(&mut blending_settings);

        // Vulkan spec defines this to be equal to true
        if blending_settings.pipeline_settings.logic_op_enable == vk::TRUE {
            // We are using logic op based blending
            // Using an enum here doesn't save much
            let blend = ColorBlendingType::BitwiseBlending(pipeline_blend_settings);
            self.color_blending = Some(blend);
        } else {
            // we are using attachment based blending
            // TODO: We currently destructure here to avoid borrow issues
            // TODO: We can avoid this by lending the same structures initially rather than retrieving them from the structure
            let ColorBlendingSettings {
                attachments,
                constants,
                ..
            } = blending_settings;
            let blend = ColorBlendingType::BlendWithAttachments(
                pipeline_blend_settings,
                attachments,
                constants,
            );
            self.color_blending = Some(blend);
        }
        // TODO: Warn when assigning values to logic op but not enabling bitwise color blending
    }
}

// TODO: Allow creation of framebuffer image attachments or accept a list of framebuffer image attachments
impl<'a, 'b: 'a> ColorBlendingSettings<'a, 'b> {
    pub fn new(
        blend_settings: &'a mut vk::PipelineColorBlendStateCreateInfoBuilder<'b>,
    ) -> ColorBlendingSettings<'a, 'b> {
        ColorBlendingSettings {
            pipeline_settings: blend_settings,
            attachments: Vec::new(),
            constants: [0.0; 4],
        }
    }

    /// Use bitwise operations to blend the two colours together, enabling this will disable blending by framebuffer attachments
    pub fn enable_bitwise_blending(&mut self, enable_blend: bool) {
        // The second method of blending colors
        self.pipeline_settings.logic_op_enable(enable_blend);
    }

    pub fn logic_blending_op(&mut self, operation: vk::LogicOp) {
        self.pipeline_settings.logic_op(operation);
    }

    pub fn set_blend_constants(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.constants[0] = r;
        self.constants[1] = g;
        self.constants[2] = b;
        self.constants[3] = a;
    }

    pub fn add_blend_attachment(
        &mut self,
        attachment: vk::PipelineColorBlendAttachmentStateBuilder<'b>,
    ) {
        // add the factor and add the attachment
        // TODO: All attachments must be the same unless independant blending is active
        // TODO: Easier way to add color blending attachments
        self.attachments.push(attachment);
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
