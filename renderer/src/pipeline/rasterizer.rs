use crate::error;

use super::traits::{ConfigureMultisampling, ConfigureRasterizer};
use super::{ConfigurePipeline, RasterizerSettings};
use error::{Error, ErrorKind};
use erupt::vk1_0 as vk;

impl<'a> ConfigureRasterizer for ConfigurePipeline<'a> {
    fn configure_rasterizer(
        &mut self,
        configure_rasterizer: &mut dyn FnMut(&mut RasterizerSettings),
    ) -> &mut dyn ConfigureMultisampling {
        let mut builder = vk::PipelineRasterizationStateCreateInfoBuilder::new();
        // Many of these options require the use of device features
        let mut config = RasterizerSettings::new(&mut builder);
        configure_rasterizer(&mut config);
        self.rasterizer_configuration = Some(builder);
        self
    }
}

impl<'a, 'b: 'a> RasterizerSettings<'a, 'b> {
    // Many of these functions require a GPU feature to have been enabled
    /// Determines how fragments are generated for geometry.
    pub fn polygon_mode(&mut self, polygon_mode: vk::PolygonMode) {
        self.settings.polygon_mode(polygon_mode);
    }
    /// If depthClampEnable is set to true, then fragments that are beyond the near and far planes
    /// are clamped to them as opposed to discarding them
    pub fn depth_clamp(&mut self, depth_clamp: bool) {
        self.settings.depth_clamp_enable(depth_clamp);
    }

    /// Discards the primivites immediately before rasterization, effectively not drawing them
    pub fn rasterizer_discard(&mut self, rasterizer_discard: bool) {
        self.settings.rasterizer_discard_enable(rasterizer_discard);
    }
    /// Describes the thickness of lines in terms of number of fragments, a value above 1.0 requires a GPU feature
    pub fn line_width(&mut self, line_width: f32) {
        self.settings.line_width(line_width);
    }
    /// Type of face culling to use
    pub fn cull_mode(&mut self, cull_mode: vk::CullModeFlags) {
        self.settings.cull_mode(cull_mode);
    }
    /// Specifies the vertex order for faces to be considered front-facing and can be clockwise or counterclockwise
    pub fn front_face(&mut self, front_face: vk::FrontFace) {
        self.settings.front_face(front_face);
    }
    /// Alter the depth values by adding a constant value or biasing them based on a fragment's slope
    pub fn depth_bias(&mut self, depth_bias: bool) {
        self.settings.depth_bias_enable(depth_bias);
    }

    pub fn depth_bias_constant_factor(&mut self, depth_bias_constant_factor: f32) {
        self.settings
            .depth_bias_constant_factor(depth_bias_constant_factor);
    }

    pub fn depth_bias_clamp(&mut self, depth_bias_clamp: f32) {
        self.settings.depth_bias_clamp(depth_bias_clamp);
    }

    pub fn depth_bias_slope_factor(&mut self, depth_bias_slope_factor: f32) {
        self.settings
            .depth_bias_slope_factor(depth_bias_slope_factor);
    }

    pub fn new(
        rasterizer_settings: &'a mut vk::PipelineRasterizationStateCreateInfoBuilder<'b>,
    ) -> RasterizerSettings<'a, 'b> {
        RasterizerSettings {
            settings: rasterizer_settings,
        }
    }

    // TODO: Change this to edit an already existing PipelineRasterizationStateCreateInfoBuilder
    // pub fn build_rasterizer<'a>(self) -> Result<(), Error> {
    //     // let ConfigureRasterizer {cull_mode, depth_bias, depth_bias_clamp, depth_bias_constant_factor, depth_bias_slope_factor, depth_clamp, front_face, line_width, polygon_mode, rasterizer_discard} = self;
    //     // TODO: Unwrap all the fields or return None?
    //     let config = self;
    //     let cull_mode = config.cull_mode.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
    //     let depth_clamp = config.depth_clamp.ok_or(Error::new(ErrorKind::InvalidPipelineConfig, None))?;
    //     // let depth_bias_constant_factor = self.depth_bias_constant_factor.ok_or(Error::InvalidRasterizationConfig("Depth Clamp was not set during the fixed function rasterization stage", self))?;
    //     let rasterizer = vk::PipelineRasterizationStateCreateInfoBuilder::new()
    //         .cull_mode(cull_mode)
    //         .depth_clamp_enable(depth_clamp)
    //         .depth_bias_constant_factor(config.depth_bias_constant_factor);
    //     Ok(rasterizer)
    // }
}
