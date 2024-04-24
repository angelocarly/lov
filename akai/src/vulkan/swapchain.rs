use std::sync::Arc;
use ash::khr::swapchain;
use ash::vk;
use ash::vk::{CompositeAlphaFlagsKHR, ImageUsageFlags, SharingMode};
use crate::vulkan::{Device, Instance, Surface};
use crate::window::Window;

pub struct Swapchain {
    swapchain_loader: swapchain::Device,
    swapchain: vk::SwapchainKHR
}

impl Swapchain {
    pub fn new(instance: Arc<Instance>, physical_device: &vk::PhysicalDevice, device: Arc<Device>, window: &Window, surface: Arc<Surface>) -> Swapchain {
        let swapchain_loader = swapchain::Device::new(instance.get_vk_instance(), device.get_vk_device());

        let surface_format = surface.get_formats(physical_device)[0];
        let surface_capabilities = surface.get_surface_capabilities(physical_device);

        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if desired_image_count > surface_capabilities.max_image_count {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let pre_transform = if surface_capabilities.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY) {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = surface.get_present_modes(physical_device);
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX )
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => window.get_extent(),
            _ => surface_capabilities.current_extent
        };

        let create_info = vk::SwapchainCreateInfoKHR::default()
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_extent(surface_resolution)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(pre_transform)
            .present_mode(present_mode)
            .min_image_count(desired_image_count)
            .surface(*surface.get_vk_surface())
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None).unwrap() };

        Self {
            swapchain_loader,
            swapchain
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe { self.swapchain_loader.destroy_swapchain(self.swapchain, None) }
    }
}