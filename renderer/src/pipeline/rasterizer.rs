use crate::error;

use super::traits::ConfigureRasterizer;
use super::{ConfigurePipeline, RasterizerSettings};
use erupt::vk1_0 as vk;
use error::{Error, ErrorKind};

impl<'a> ConfigureRasterizer for ConfigurePipeline<'a> {
    fn configure_rasterizer(
        &mut self,
        configure_rasterizer: &mut dyn FnMut(&mut RasterizerSettings),
    ) {
        // Many of these options require the use of device features
        let mut config = RasterizerSettings::new();
        configure_rasterizer(&mut config);

        let builder = vk::PipelineRasterizationStateCreateInfoBuilder::new();
        // TODO: Helper struct that can determine if certain options are available and allow customizing those options conditionally
        // TODO: Although you are more likely to create a different pipeline altogether that modify one conditionally?
        config.build_rasterizer();
        builder.depth_bias_clamp(0.0);
        builder.depth_bias_enable(false);
        builder.polygon_mode(vk::PolygonMode::FILL);
        builder.cull_mode(vk::CullModeFlags::BACK);
        builder.rasterizer_discard_enable(false);
        builder.line_width(1.);
        //self
    }
}

impl RasterizerSettings {
    pub fn polygon_mode(&mut self, polygon_mode: vk::PolygonMode) {
        self.polygon_mode = Some(polygon_mode);
    }

    pub fn depth_clamp(&mut self, depth_clamp: bool) {
        self.depth_clamp = Some(depth_clamp);
    }

    pub fn rasterizer_discard(&mut self, rasterizer_discard: bool) {
        self.rasterizer_discard = Some(rasterizer_discard);
    }

    pub fn line_width(&mut self, line_width: f32) {
        self.line_width = Some(line_width);
    }

    pub fn cull_mode(&mut self, cull_mode: vk::CullModeFlags) {
        self.cull_mode = Some(cull_mode);
    }

    pub fn front_face(&mut self, front_face: vk::FrontFace) {
        self.front_face = Some(front_face);
    }

    pub fn depth_bias(&mut self, depth_bias: bool) {
        self.depth_bias = Some(depth_bias);
    }

    pub fn depth_bias_constant_factor(&mut self, depth_bias_constant_factor: f32) {
        self.depth_bias_constant_factor = depth_bias_constant_factor;
    }

    pub fn depth_bias_clamp(&mut self, depth_bias_clamp: f32) {
        self.depth_bias_clamp = depth_bias_clamp;
    }

    pub fn depth_bias_slope_factor(&mut self, depth_bias_slope_factor: f32) {
        self.depth_bias_slope_factor = depth_bias_slope_factor;
    }

    pub fn new() -> RasterizerSettings {
        RasterizerSettings {
            polygon_mode: None,
            rasterizer_discard: None,
            depth_clamp: None,
            line_width: None,
            cull_mode: None,
            front_face: None,
            depth_bias: None,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
        }
    }

    pub fn build_rasterizer(self) -> Result<vk::PipelineRasterizationStateCreateInfo, Error> {
        // let ConfigureRasterizer {cull_mode, depth_bias, depth_bias_clamp, depth_bias_constant_factor, depth_bias_slope_factor, depth_clamp, front_face, line_width, polygon_mode, rasterizer_discard} = self;
        // TODO: Unwrap all the fields or return None?
        let config = self;
        let cull_mode = config.cull_mode.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
        let depth_clamp = config.depth_clamp.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
        // let depth_bias_constant_factor = self.depth_bias_constant_factor.ok_or(Error::InvalidRasterizationConfig("Depth Clamp was not set during the fixed function rasterization stage", self))?;
        let rasterizer = vk::PipelineRasterizationStateCreateInfoBuilder::new()
            .cull_mode(cull_mode)
            .depth_clamp_enable(depth_clamp)
            .depth_bias_constant_factor(config.depth_bias_constant_factor)
            .build();
        Ok(rasterizer)
    }
}
