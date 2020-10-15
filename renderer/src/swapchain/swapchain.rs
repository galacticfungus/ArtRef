use erupt::vk1_0 as vk;
use super::Swapchain;

impl<'a> Drop for Swapchain<'a> {
    fn drop(&mut self) {
        println!("Swapchain is being dropped");
        
    }
}

impl<'a> Swapchain<'a> {
    pub fn total_images(&self) -> usize {
        self.images.len()
    }

    pub fn format(&self) -> vk::Format {
        self.surface_format.format
    }

    pub fn get_image(&self, index: usize) -> &vk::Image {
        &self.images[index]
    }

    pub fn get_images(&self) -> &[vk::Image] {
        self.images.as_slice()
    }
}

impl<'a> Into<erupt::extensions::khr_swapchain::SwapchainKHR> for Swapchain<'a> {
    fn into(self) -> erupt::extensions::khr_swapchain::SwapchainKHR {
        self.swapchain
    }
}