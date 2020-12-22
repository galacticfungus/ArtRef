use super::{
    ConfigurePresenter, PresentMode, Presenter, SurfaceColourSpace, SurfaceFormat, SwapchainExtent,
    SwapchainImageCount,
};
use crate::{
    error::{Error, ErrorKind},
    PickManager,
};
use erupt::extensions::khr_surface as surface;
use erupt::vk1_0 as vk;

impl ConfigurePresenter {
    pub fn new(
        surface: surface::SurfaceKHR,
        surface_capabilities: surface::SurfaceCapabilitiesKHR,
        surface_formats: Vec<surface::SurfaceFormatKHR>,
        present_modes: Vec<surface::PresentModeKHR>,
    ) -> Self {
        Self {
            surface,
            surface_capabilities,
            present_modes,
            surface_formats,
            swapchain_extent: None,
            image_count: None,
            surface_format: None,
            present_mode: None,
        }
    }

    /// The mode picked is the first one available that is added to the list, The default mode is Fifo as that mode is guarenteed to be available
    pub fn select_present_mode<F>(mut self, select_mode: F) -> Result<Self, Error>
    where
        F: Fn(&mut PickManager<PresentMode, surface::PresentModeKHR>) -> (),
    {
        let mut modes_picked = Vec::new();
        let present_mode = self
            .present_mode
            .get_or_insert(surface::PresentModeKHR::default());
        let mut picker: PickManager<PresentMode, surface::PresentModeKHR> = PickManager::new(
            &mut modes_picked,
            present_mode,
            self.present_modes.as_slice(),
        );
        select_mode(&mut picker);
        picker.get_first_available();
        // TODO: Remove the result since this can never return an error?
        Ok(self)
    }

    pub fn select_surface_format<F>(mut self, pick_surface_format: F) -> Self
    where
        F: Fn(&mut PickManager<SurfaceFormat, vk::Format>) -> (),
    {
        let surface_format = self
            .surface_format
            .get_or_insert(surface::SurfaceFormatKHR::default());
        //let data = &mut surface_format.format;
        let mut formats_picked: Vec<SurfaceFormat> = Vec::new();
        let available_formats: Vec<vk::Format> = self
            .surface_formats
            .iter()
            .map(|format| format.format)
            .collect();
        let mut picker = PickManager::new(
            &mut formats_picked,
            &mut surface_format.format,
            available_formats.as_slice(),
        );
        pick_surface_format(&mut picker);
        println!("After picking format it's {:?}", picker);
        picker.get_first_available();

        self
    }
    /// Selects the colour space that will be used on the surface and swapchain, the first valid item is picked, unsupported formats are ignored
    /// sRGB is the default colour space format as all implementations must support it
    pub fn select_surface_colour_space<F>(mut self, colour_space: F) -> Self
    where
        F: Fn(&mut PickManager<SurfaceColourSpace, surface::ColorSpaceKHR>) -> (),
    {
        // TODO: Enable selecting a colour space based on characteristics of that colour space, ie alpha channel 4 bytes etc
        let surface_format = self
            .surface_format
            .get_or_insert(surface::SurfaceFormatKHR::default());
        let mut colour_spaces_picked = Vec::new();
        let available_colour_spaces: Vec<surface::ColorSpaceKHR> = self
            .surface_formats
            .iter()
            .map(|format| format.color_space)
            .collect();
        let mut picker = PickManager::new(
            &mut colour_spaces_picked,
            &mut surface_format.color_space,
            available_colour_spaces.as_slice(),
        );
        colour_space(&mut picker);
        picker.get_first_available();
        self
    }

    /// This function only calls custom_extent on some platforms, on other platforms the resolution of the surface is set when it is created
    pub fn select_extent<F>(mut self, custom_extent: F) -> Self
    where
        F: Fn(&mut SwapchainExtent) -> (),
    {
        if self.surface_capabilities.current_extent.width == std::u32::MAX {
            let mut swapchain_extent = SwapchainExtent::new(
                &self.surface_capabilities.min_image_extent,
                &self.surface_capabilities.max_image_extent,
            );
            // if extent is 0,0 then window is minimized or hidden, basically it's surface is currently unavailable
            custom_extent(&mut swapchain_extent);
        // TODO: Set extent to the returned extent
        } else {
            // The size of the window defines the resolution of the swapchain
            self.swapchain_extent = Some(self.surface_capabilities.current_extent);
        }
        self
    }

    /// The number of images to use for the swapchain
    pub fn select_presentation_image_count<F: Fn(&mut SwapchainImageCount) -> ()>(
        mut self,
        select_image_count: F,
    ) -> Self {
        let mut swapchain_image_count = SwapchainImageCount::new(
            self.surface_capabilities.min_image_count,
            self.surface_capabilities.max_image_count,
        );
        select_image_count(&mut swapchain_image_count);
        self.image_count = Some(SwapchainImageCount::image_count(swapchain_image_count));
        self
    }

    /// Select the ImageViews that are used in conjunction with the swapchain
    pub fn select_presentation_view_type<F: Fn(&mut u32) -> ()>(
        mut self,
        select_view_settings: F,
    ) -> Self {
        // TODO: Do the presentation image views ever really change
        // NOTE: This is configuring the image types for presentation only
        // let mut settings = ImageViewSettings::new();
        // select_view_settings(&mut settings);
        // This should be a simple VkImageViewCreateInfo
        // self.presentation_image_view_settings = Some(settings);
        self
    }

    // Ensure optional values have a value
    fn validate_swapchain(
        &self,
    ) -> Result<
        (
            u32,
            surface::SurfaceFormatKHR,
            vk::Extent2D,
            surface::PresentModeKHR,
        ),
        Error,
    > {
        let image_count = self
            .image_count
            .ok_or(Error::new(ErrorKind::SwapchainConfigurationMissing, None))?;
        let present_mode = self
            .present_mode
            .ok_or(Error::new(ErrorKind::SwapchainConfigurationMissing, None))?;
        let surface_format = self
            .surface_format
            .ok_or(Error::new(ErrorKind::SwapchainConfigurationMissing, None))?;
        let extent = self
            .swapchain_extent
            .ok_or(Error::new(ErrorKind::SwapchainConfigurationMissing, None))?;
        Ok((image_count, surface_format, extent, present_mode))
    }

    fn validate_presentation_image_views(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn create_presenter(self) -> Presenter {
        Presenter::from(self)
    }
}
