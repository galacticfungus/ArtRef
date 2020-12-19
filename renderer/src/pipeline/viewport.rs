use super::traits::ConfigureViewport;
use super::{traits::ConfigureRasterizer, ConfigurePipeline, Viewport, ViewportManager};

use erupt::vk1_0 as vk;

impl<'a> ConfigureViewport for ConfigurePipeline<'a> {
    fn configure_viewport(
        &mut self,
        create_viewport: &mut dyn FnMut(&mut ViewportManager),
    ) -> &mut dyn ConfigureRasterizer {
        // It's possible to create multiple viewports but its locked behind a gpu feature,
        // Each viewport has a scissor associated with it
        let mut mng = ViewportManager::new();
        create_viewport(&mut mng);

        // VkPipelineViewportStateCreateInfo viewportState{};
        // viewportState.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
        // viewportState.viewportCount = 1;
        // viewportState.pViewports = &viewport;
        // viewportState.scissorCount = 1;
        // viewportState.pScissors = &scissor;
        // TODO: Verify a viewport was created ie if len is 0 then leave it as None?
        self.viewports_to_create = Some(mng.viewports);
        self
    }
}

impl ViewportManager {
    pub fn new() -> ViewportManager {
        ViewportManager {
            viewports: Vec::new(),
        }
    }
    pub fn create_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
        scissor_x: i32,
        scissor_y: i32,
        scissor_width: u32,
        scissor_height: u32,
    ) {
        // TODO: Check that mutiple viewports are supported
        // Width and Height are limited to the max values of a framebuffer
        let viewport = Viewport::new(
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
            scissor_x,
            scissor_y,
            scissor_width,
            scissor_height,
        );
        self.viewports.push(viewport);
    }
}

impl Viewport {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
        scissor_x: i32,
        scissor_y: i32,
        scissor_width: u32,
        scissor_height: u32,
    ) -> Viewport {
        Viewport {
            viewport: vk::ViewportBuilder::new()
                .height(height)
                .width(width)
                .x(x)
                .y(y)
                .min_depth(min_depth)
                .max_depth(max_depth)
                .build(),
            scissor: vk::Rect2DBuilder::new()
                .extent(
                    vk::Extent2DBuilder::new()
                        .height(scissor_height)
                        .width(scissor_width)
                        .build(),
                )
                .offset(vk::Offset2DBuilder::new().x(scissor_x).y(scissor_y).build())
                .build(),
        }
    }
}
