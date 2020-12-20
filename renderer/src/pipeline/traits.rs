use super::{RasterizerSettings, VertexInputSettings, ViewportManager, InputAssembelySettings};
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
        configure_assembely: &mut FnMut(&mut InputAssembelySettings),
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
    );
}
