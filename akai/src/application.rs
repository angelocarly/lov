use std::sync::Arc;
use ash::vk::PhysicalDevice;
use winit::event::Event;
use winit::event_loop::EventLoop;
use crate::vulkan::{Device, Instance, Surface, Swapchain};
use crate::window::Window;

/// Generative art runtime.
/// Manages the window and graphics recording.
pub struct Application {
    pub swapchain: Arc<Swapchain>,

    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub entry: Arc<ash::Entry>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
    pub physical_device: PhysicalDevice,
    pub instance: Arc<Instance>,
}

impl Application {
    pub fn new(window_title: &str, width: u32, height: u32) -> Application {
        let event_loop = EventLoop::new().expect("Failed to create event loop.");
        let window = Window::create(&event_loop, window_title, width, height);

        let entry = Arc::new(ash::Entry::linked());
        let instance = Arc::new(Instance::new(entry.clone(), window.display_handle()));
        let surface = Arc::new(Surface::new(instance.clone(), &window));
        let (physical_device, queue_family_index) = instance.create_physical_device(surface.clone());
        let device = Arc::new(Device::new(instance.clone(), physical_device, queue_family_index));

        let swapchain = Arc::new(Swapchain::new(instance.clone(), &physical_device, device.clone(), &window, surface.clone()));

        Self {
            event_loop,
            window,
            entry,
            instance,
            surface,
            physical_device,
            device,
            swapchain
        }
    }

    pub fn run(mut self) {
        self.event_loop
            .run(|event, elwt| if let Event::WindowEvent { event, .. } = event {
                self.window.window_event(event, elwt);
            })
            .unwrap()
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new("Akai engine", 800, 600)
    }
}