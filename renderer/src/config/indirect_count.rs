use std::ffi::CStr;
use ash::vk;
use ash::version::{DeviceV1_0, InstanceV1_0};
use std::mem;

#[derive(Clone)]
pub struct DrawIndirectCount {
    indirect_count_fn: vk::KhrDrawIndirectCountFn,
}

impl DrawIndirectCount {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> DrawIndirectCount {
        let indirect_count_fn = vk::KhrDrawIndirectCountFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        DrawIndirectCount {
            indirect_count_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrDrawIndirectCountFn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html>"]
    pub unsafe fn cmd_draw_indirect_count(&self, command_buffer: vk::CommandBuffer, buffer: vk::Buffer, offset: vk::DeviceSize, count_buffer: vk::Buffer, count_buffer_offset: vk::DeviceSize, max_draw_count: u32, stride: u32) {
        self.indirect_count_fn.cmd_draw_indirect_count_khr(command_buffer, buffer, offset, count_buffer, count_buffer_offset, max_draw_count, stride);
    }
    
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html>"]
    pub unsafe fn cmd_draw_indexed_indirect_count(&self, command_buffer: vk::CommandBuffer, buffer: vk::Buffer, offset: vk::DeviceSize, count_buffer: vk::Buffer, count_buffer_offset: vk::DeviceSize, max_draw_count: u32, stride: u32) {
        self.indirect_count_fn.cmd_draw_indexed_indirect_count_khr(command_buffer, buffer, offset, count_buffer, count_buffer_offset, max_draw_count, stride);
    }
}