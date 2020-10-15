use erupt::vk1_0 as vk;
use erupt::extensions::khr_swapchain;
use super::{Swapchain, VulkanDevice};

pub struct RenderDevice<'a> {
    device: VulkanDevice,
    swapchain: Swapchain<'a>,
    image_views: Vec<vk::ImageView>,
}

impl<'a> RenderDevice<'a> {
    pub fn new(device: VulkanDevice, swapchain: Swapchain) -> RenderDevice {
        // let image_views: Vec<vk::ImageView> = Vec::with_capacity(swapchain.);
        // TODO: Iterators are cool
        let image_views = swapchain.get_images().iter().map(|image| {
            let create_info = vk::ImageViewCreateInfoBuilder::new()
                .format(swapchain.format())
                // TODO: This should be configurable
                .view_type(vk::ImageViewType::_2D)
                // TODO: Creating the images can be done when building the swapchain
                
                // image_view_builder.image(swapchain.get_image(0));
                .image(*image)
                // TODO: THis should be configurable
                .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                })
                // TODO: This should be configurable
                .subresource_range(vk::ImageSubresourceRangeBuilder::new()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build())
                .build();
            unsafe {device.handle().create_image_view(&create_info, None, None)}
                .expect("Failed to create image view from image while initializing render device")
        }).collect::<Vec<vk::ImageView>>();
        
        RenderDevice {
            device,
            swapchain,
            image_views,
        }
    }
}

// Load shaders and create shader modules
// Create renderpass
// Create graphics pipeline - This is BIG
// Create framebuffers

// Create memory buffers

impl<'a> Drop for RenderDevice<'a> {
    fn drop(&mut self) {
        for image_view in self.image_views.iter() {
            unsafe {self.device.handle().destroy_image_view(Some(*image_view), None)};
        }
    }
}