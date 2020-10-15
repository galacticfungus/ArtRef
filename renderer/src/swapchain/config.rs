use super::{ConfigureSwapchain, Swapchain, SwapchainExtent};
use erupt::extensions::khr_surface as surface;
use erupt::extensions::khr_swapchain as swapchain;
use erupt::vk1_0 as vk;

use crate::{
    error, PickManager, PresentMode, Surface, SurfaceColourSpace, SurfaceFormat, VulkanDevice,
};

impl<'a, 'b> ConfigureSwapchain<'a, 'b> {
    pub fn new(
        instance: &erupt::InstanceLoader,
        device: &'a VulkanDevice,
        surface: Surface<'b>,
    ) -> ConfigureSwapchain<'a, 'b> {
        //device.handle().create_swapchain_khr() = device.create_swapchain
        ConfigureSwapchain {
            surface_format: None,
            present_mode: None,
            present_modes: device.get_present_modes(),
            surface_formats: device.get_surface_formats(),
            surface_capabilities: device.get_surface_capabilities(),
            surface,
            swapchain_extent: None,
            image_count: None,
            device,
        }
    }

    /// The mode picked is the first one available that is added to the list, The default mode is Fifo as that mode is guarenteed to be available
    pub fn select_present_mode<F>(&mut self, select_mode: F) -> Result<&mut Self, error::Error>
    where
        F: Fn(&mut PickManager<PresentMode, surface::PresentModeKHR>) -> (),
    {
        let mut modes_picked = Vec::new();
        let present_mode = self
            .present_mode
            .get_or_insert(surface::PresentModeKHR::default());
        let mut picker: PickManager<PresentMode, surface::PresentModeKHR> =
            PickManager::new(
                &mut modes_picked, 
                present_mode, 
                self.present_modes);
        select_mode(&mut picker);
        picker.get_first_available();
        Ok(self)
    }

    pub fn select_surface_format<F>(&mut self, pick_surface_format: F) -> &mut Self
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
        let mut picker =
            PickManager::new(
                &mut formats_picked, 
                &mut surface_format.format, 
                available_formats.as_slice());
        pick_surface_format(&mut picker);
        println!("After picking format it's {:?}", picker);
        picker.get_first_available();
        
        self
    }

    pub fn select_surface_colour_space<F>(&mut self, colour_space: F) -> &mut Self
    where
        F: Fn(&mut PickManager<SurfaceColourSpace, surface::ColorSpaceKHR>) -> (),
    {
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
    pub fn select_extent<F>(&mut self, custom_extent: F) -> &mut Self
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

    pub fn select_image_count<F: Fn(&mut SwapchainImageCount) -> ()>(
        &mut self,
        select_image_count: F,
    ) -> &mut Self {
        let mut image_count = SwapchainImageCount::new(
            self.surface_capabilities.min_image_count,
            self.surface_capabilities.max_image_count,
        );
        select_image_count(&mut image_count);
        self.image_count = Some(image_count.image_count);
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
        error::Error,
    > {
        let image_count = self
            .image_count
            .ok_or(error::Error::SwapchainConfigurationMissing("Image Count"))?;
        let present_mode = self
            .present_mode
            .ok_or(error::Error::SwapchainConfigurationMissing("Present Mode"))?;
        let surface_format =
            self.surface_format
                .ok_or(error::Error::SwapchainConfigurationMissing(
                    "Surface Format",
                ))?;
        let extent = self
            .swapchain_extent
            .ok_or(error::Error::SwapchainConfigurationMissing(
                "Swapchain Extent",
            ))?;
        Ok((image_count, surface_format, extent, present_mode))
    }

    pub fn build(self) -> Result<Swapchain<'b>, error::Error> {
        let (image_count, surface_format, extent, present_mode) = self.validate_swapchain()?;
        let create_info_builder = swapchain::SwapchainCreateInfoKHRBuilder::new();
        println!("Format and Image Space: {:?} and {:?}",surface_format.format, surface_format.color_space);
        //vk::Format::R8G8B8A8_SRGB
        let create_info = create_info_builder
            .image_format(surface_format.format)
            .clipped(true)
            .composite_alpha(surface::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
            .image_array_layers(1)
            .min_image_count(image_count)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .pre_transform(self.surface_capabilities.current_transform)
            .present_mode(present_mode)
            .old_swapchain(swapchain::SwapchainKHR::null())
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .surface(self.surface.platform_surface)
            .build();
        let internal_swapchain = unsafe {
            self.device
                .handle()
                .create_swapchain_khr(&create_info, None, None)
        }
        .expect("Failed to create underlying swapchain");
        let swapchain_images = unsafe {self.device.handle().get_swapchain_images_khr(internal_swapchain, None)}
            .expect("Failed to create swapchain images");
        let swapchain = Swapchain {
            image_count: self.image_count.unwrap(),
            present_mode: self.present_mode.unwrap(),
            surface_format: self.surface_format.unwrap(),
            surface: self.surface,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            swapchain_extent: self.swapchain_extent.unwrap(),
            transform: self.surface_capabilities.current_transform,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            swapchain: internal_swapchain,
            previous_swapchain: None,
            clipped: true,
            composite_alpha: surface::CompositeAlphaFlagsKHR::OPAQUE_KHR,
            images: swapchain_images,
        };
        Ok(swapchain)
    }
}

pub struct SwapchainImageCount {
    min_images: u32,
    max_images: u32,
    image_count: u32,
}

impl SwapchainImageCount {
    pub fn new(min_images: u32, max_images: u32) -> SwapchainImageCount {
        SwapchainImageCount {
            min_images,
            max_images,
            image_count: 0,
        }
    }

    pub fn min_images(&self) -> u32 {
        self.min_images
    }

    pub fn max_images(&self) -> u32 {
        self.max_images
    }

    pub fn set_image_count(&mut self, image_count: u32) {
        self.image_count = image_count;
    }
}
