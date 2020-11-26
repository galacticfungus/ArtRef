use erupt::DeviceLoader;
use erupt::vk1_0 as vk;
pub struct Renderpass<'a, 'b> {
    device: &'a DeviceLoader,
    attachments: Vec<&'b Attachment>,
    render_pass: Option<vk::RenderPass>
}

impl<'a, 'b> Renderpass<'a, 'b> {
    // TODO: Subpass instead of attachment although each attachment is used in a subpass
    // pub fn add_subpass(mut self, format: vk::Format) -> Self {
    //     // TODO: Each subpass can have multiple attachments
    //     // TODO: Multiple subpasses can reference the same attachments as inputs or outputs
    //     // TODO: So we build a list of render passes and then reference the attachments as needed
    //     let subpass_builder = vk::SubpassDescriptionBuilder::new() 
    //         .color_attachments(color_attachments)
    //         Attachments used for multisampling color attachments
    //         .resolve_attachments(resolve_attachments)
    //         Attachments that are not used by this subpass, but for which the data must be preserved
    //         .preserve_attachments(preserve_attachments)
    //         .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
    //         Attachments that are read from a shader
    //         .input_attachments(input_attachments)
    //         Attachment for depth and stencil data
    //         .depth_stencil_attachment(depth_stencil_attachment)
    //         .flags(vk::SubpassDescriptionFlags::SHADER_RESOLVE_QCOM);
    //     let reference_builder = vk::AttachmentReferenceBuilder::new()
    //         .attachment(0)
    //         .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
    //     self
    // }

    // pub fn create_attachment(mut self) -> Self {
    //     let attach_builder = vk::AttachmentDescriptionBuilder::new()
    //         .format(format)
    //         .store_op(vk::AttachmentStoreOp::STORE)
    //         .samples(vk::SampleCountFlagBits::_1)
    //         .flags(vk::AttachmentDescriptionFlags::MAY_ALIAS)
    //         .load_op(vk::AttachmentLoadOp::DONT_CARE)
    //         .stencil_load_op(stencil_load_op)
    //         .stencil_store_op(stencil_store_op)
    //         .final_layout(final_layout);
    //     self
    // }

    pub fn create_test_renderpass(&mut self, swapchain_format: vk::Format) {
        // VkAttachmentDescription colorAttachment{};
        // colorAttachment.format = swapChainImageFormat;
        // colorAttachment.samples = VK_SAMPLE_COUNT_1_BIT;
        // colorAttachment.loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR;
        // colorAttachment.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
        // colorAttachment.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        // colorAttachment.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
        // colorAttachment.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        // colorAttachment.finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        let color_attachment = vk::AttachmentDescriptionBuilder::new()
            .format(swapchain_format)
            .samples(vk::SampleCountFlagBits::_1)
            .load_op(vk::AttachmentLoadOp::LOAD)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        // VkAttachmentReference colorAttachmentRef{};
        // colorAttachmentRef.attachment = 0;
        // colorAttachmentRef.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        let reference = vk::AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        // VkSubpassDescription subpass{};
        // subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
        // subpass.colorAttachmentCount = 1;
        // subpass.pColorAttachments = &colorAttachmentRef;
        let subpass = vk::SubpassDescriptionBuilder::new()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&reference));
        // VkRenderPassCreateInfo renderPassInfo{};
        // renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
        // renderPassInfo.attachmentCount = 1;
        // renderPassInfo.pAttachments = &colorAttachment;
        // renderPassInfo.subpassCount = 1;
        // renderPassInfo.pSubpasses = &subpass;
        let sexy = std::slice::from_ref(&color_attachment);
        let renderpass_create_info = vk::RenderPassCreateInfoBuilder::new()
            .attachments(sexy)
            .subpasses(std::slice::from_ref(&subpass));
        
        // if (vkCreateRenderPass(device, &renderPassInfo, nullptr, &renderPass) != VK_SUCCESS) {
        //     throw std::runtime_error("failed to create render pass!");
        // }
        let rp = unsafe { self.device.create_render_pass(&renderpass_create_info, None, None) }
            .expect("Failed to create test render pass");
        self.render_pass = Some(rp);
    }

    pub fn add_attachment_input(&mut self, attachment: &Attachment) {

    }

    pub fn add_colour_output(&mut self, attachment: &Attachment) {
        
    }

    pub fn add_presentation_output(&mut self, attachment: &Attachment) {

    }

    pub fn new(device: &DeviceLoader) -> Renderpass {
        Renderpass {
            attachments: Vec::new(),
            device,
            render_pass: None,
        }
    }

    // pub fn create_renderpass(mut self) -> Self {
    //     // TODO: We pass in an object that allows the creation of attachments and subpasses after initial config?
    //     // TODO: How to pass this info to the render graph - ie v2, create the render graph as part of the sub pass/ attachment configuration

    //     // create attachments

    //     // create sub passes using attachments

    //     // build render graph


    //     // Can render passes use dynamic attachments?

    //     // return renderpass object
    //     self
    // }
}

pub struct Attachment {
    attachment: vk::AttachmentDescription,

}

impl Attachment {
    pub fn new() -> Attachment {
        Attachment {
            attachment: vk::AttachmentDescription::default(),
        }
    }
}

pub struct Subpass {

}

// Create attachment
// VkAttachmentDescription colorAttachment{};
//     colorAttachment.format = swapChainImageFormat;
//     colorAttachment.samples = VK_SAMPLE_COUNT_1_BIT;
// VkSubpassDescription subpass{};
    // subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
// Create renderpass based on attachments created
// VkRenderPassCreateInfo renderPassInfo{};
// renderPassInfo.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
// renderPassInfo.attachmentCount = 1;
// renderPassInfo.pAttachments = &colorAttachment;
// renderPassInfo.subpassCount = 1;
// renderPassInfo.pSubpasses = &subpass;



// if (vkCreateRenderPass(device, &renderPassInfo, nullptr, &renderPass) != VK_SUCCESS) {
//     throw std::runtime_error("failed to create render pass!");
// }