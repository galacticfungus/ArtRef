use super::traits::ConfigureDepthStencil;
use super::DepthStencilSettings;
use crate::ConfigurePipeline;
use erupt::vk1_0 as vk;

// TODO: This is not required so can be completely skipped

impl<'a> ConfigureDepthStencil for ConfigurePipeline<'a> {
    fn configure_depthstencil(
        &mut self,
        configure_depthstencil: &mut dyn FnMut(&mut DepthStencilSettings),
    ) -> &mut dyn super::traits::ConfigureColorBlending {
        let mut pipeline_state = vk::PipelineDepthStencilStateCreateInfoBuilder::new();
        let mut settings = DepthStencilSettings::new(&mut pipeline_state);
        configure_depthstencil(&mut settings);
        self
    }
}

impl<'a, 'b: 'a> DepthStencilSettings<'a, 'b> {
    pub fn new(
        settings: &'a mut vk::PipelineDepthStencilStateCreateInfoBuilder<'b>,
    ) -> DepthStencilSettings<'a, 'b> {
        DepthStencilSettings { settings }
    }

    pub fn enable_stencil_test(&mut self, enable_test: bool) {
        self.settings.stencil_test_enable(enable_test);
    }

    pub fn min_depth_bounds(&mut self, min_bounds: f32) {
        self.settings.min_depth_bounds(min_bounds);
    }

    pub fn max_depth_bounds(&mut self, max_bounds: f32) {
        self.settings.max_depth_bounds(max_bounds);
    }

    pub fn front(&mut self, stencil_state: vk::StencilOpState) {
        self.settings.front(stencil_state);
    }

    pub fn depth_writable(&mut self, is_writable: bool) {
        self.settings.depth_write_enable(is_writable);
    }

    pub fn depth_compare(&mut self, compare_type: vk::CompareOp) {
        self.settings.depth_compare_op(compare_type);
    }

    pub fn enable_depth_test(&mut self, is_enabled: bool) {
        self.settings.depth_test_enable(is_enabled);
    }

    pub fn enable_depth_bounds(&mut self, enable: bool) {
        self.settings.depth_bounds_test_enable(enable);
    }
}
