use super::{ColorBlendingSettings, DepthStencilSettings, InputAssembelySettings, MultiSampleSettings, RasterizerSettings, VertexInputSettings, ViewportManager};
use erupt::vk1_0 as vk;
pub trait ConfigureVertexInput {
    fn configure_vertex_input(
        &mut self,
        configure_input: &mut dyn FnMut(&mut VertexInputSettings) -> (),
    ) -> &mut dyn ConfigureInputAssembely;
}

pub trait ConfigureInputAssembely {
    fn configure_input_assembely(
        &mut self,
        configure_assembely: &mut dyn FnMut(&mut InputAssembelySettings),
    ) -> &mut dyn ConfigureViewport;
}

pub trait ConfigureViewport {
    fn configure_viewport(
        &mut self,
        create_viewport: &mut dyn FnMut(&mut ViewportManager),
    ) -> &mut dyn ConfigureRasterizer;
}

pub trait ConfigureRasterizer {
    fn configure_rasterizer(
        &mut self,
        configure_rasterizer: &mut dyn FnMut(&mut RasterizerSettings),
    ) -> &mut dyn ConfigureMultisampling;
}

pub trait ConfigureMultisampling {
    fn configure_multisampling(&mut self, configure_multisampling: &mut dyn FnMut(&mut MultiSampleSettings)) -> &mut dyn ConfigureDepthStencil;
}

pub trait ConfigureDepthStencil {
    fn configure_depthstencil(&mut self, configure_depthstencil: &mut dyn FnMut(&mut DepthStencilSettings)) -> &mut dyn ConfigureColorBlending;
}

pub trait ConfigureColorBlending {
    fn configure_blending(&mut self, configure_blending: &mut dyn FnMut(&mut ColorBlendingSettings));
}
