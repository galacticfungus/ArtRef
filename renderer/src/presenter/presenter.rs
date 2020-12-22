use super::{ConfigurePresenter, Presenter};
use erupt::extensions::khr_surface as surface;
use erupt::vk1_0 as vk;

impl Presenter {
    pub fn get_width(&self) -> u32 {
        self.extent.width
    }

    pub fn get_height(&self) -> u32 {
        self.extent.height
    }

    pub fn get_format(&self) -> vk::Format {
        self.surface_format.format
    }

    pub fn get_color_space(&self) -> surface::ColorSpaceKHR {
        self.surface_format.color_space
    }
}

impl From<ConfigurePresenter> for Presenter {
    fn from(configuration: ConfigurePresenter) -> Self {
        let ConfigurePresenter {
            surface,
            surface_format,
            swapchain_extent,
            ..
        } = configuration;
        Presenter {
            surface: surface,
            surface_format: surface_format.unwrap(),
            extent: swapchain_extent.unwrap(),
        }
    }
}
