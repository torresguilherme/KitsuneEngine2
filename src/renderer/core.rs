use super::mesh::Mesh;

use crate::display::game_window::GameWindow;

use std::sync::Arc;
use std::collections::HashSet;

use vulkano_win::VkSurfaceBuild;

use vulkano::instance::{
    Instance,
    InstanceExtensions,
    ApplicationInfo,
    Version,
    layers_list,
    PhysicalDevice,
};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::device::{Device, DeviceExtensions, Queue, Features};
use vulkano::swapchain::{
    Surface,
    Capabilities,
    ColorSpace,
    SupportedPresentModes,
    PresentMode,
    Swapchain,
    CompositeAlpha,
    acquire_next_image,
};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::{SharingMode, GpuFuture};
use vulkano::pipeline::{
    GraphicsPipeline,
    vertex::BufferlessDefinition,
    vertex::BufferlessVertices,
    viewport::Viewport,
};
use vulkano::framebuffer::{
    RenderPassAbstract,
    Subpass,
    FramebufferAbstract,
    Framebuffer,
};
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::command_buffer::{
    AutoCommandBuffer,
    AutoCommandBufferBuilder,
    DynamicState,
};

mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
        #version 450

        layout(location = 0) in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }"
    }
}

mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
        #version 450

        layout(location = 0) out vec4 f_color;

        void main() {
            f_color = vec4(0.0, 0.0, 0.0, 1.0);
        }"
    }
}

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

struct QueueFamilyIndices {
    graphics_family: i32,
    present_family: i32
}

impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: -1,
            present_family: -1
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

pub struct Core<'a> {
    meshes: Vec<&'a Mesh>,
    // todo add materials assigned to each mesh

    // shaders
    fragment_shader: fs::Shader,
    vertex_shader: vs::Shader,

    // vulkan structures
    instance: Arc<Instance>,
    device: Arc<Device>,
    debug_callback: Option<DebugCallback>
}

impl<'a> Core<'a> {
    pub fn new() -> Core<'a> {
        // creating vulkan instance and checking for drivers
        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("not possible to create instance, do you have the vulkan drivers?");
        let physical_device = PhysicalDevice::enumerate(&instance).next().expect("there are no devices that support vulkan.");

        // enumerates family queues, creates an device associated to a family that supports graphics operations
        for family in physical_device.queue_families() {
            println!("a family queue with {:?} families was found!", family.queues_count());
        }

        let queue_family = physical_device.queue_families()
            .find(|&q| q.supports_graphics())
            .expect("error: none of the queue families support graphical operations");
        
        println!("the chosen family queue has {:?} queues", queue_family.queues_count());
        let (device, mut queues) = Device::new(physical_device, &Features::none(), &DeviceExtensions::none(), 
            [(queue_family, 0.5)].iter().cloned()).expect("error: creating the device wasn't possible!");

        // initializing shader modules
        let fragment_shader = fs::Shader::load(device.clone()).expect("failed to create shader module");
        let vertex_shader = vs::Shader::load(device.clone()).expect("failed to create shader module");

        let debug_callback = Self::setup_debug_callback(&instance);

        Core {
            meshes: Vec::new(),
            fragment_shader,
            vertex_shader,
            instance,
            device,
            debug_callback
        }
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS  {
            return None;
        }

        let msg_types = MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: false,
            debug: true,
        };
        DebugCallback::new(&instance, msg_types, |msg| {
            println!("validation layer: {:?}", msg.description);
        }).ok()
}

    pub fn add_new(&mut self, n_mesh: &'a Mesh) {
        self.meshes.push(n_mesh);
    }

    pub fn render(&self, window: &mut GameWindow) {

    }
}