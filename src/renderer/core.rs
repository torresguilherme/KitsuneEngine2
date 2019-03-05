use std::sync::Arc;

use super::mesh::Mesh;

use crate::display::game_window::GameWindow;

use vulkano::pipeline::shader::ShaderModule;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

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

pub struct Core<'a> {
    meshes: Vec<&'a Mesh>,
    // todo add materials assigned to each mesh
    fragment_shader: fs::Shader,
    vertex_shader: vs::Shader,
    instance: Arc<Instance>,
    device: Arc<Device>
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

        Core {
            meshes: Vec::new(),
            fragment_shader: fragment_shader,
            vertex_shader: vertex_shader,
            instance: instance,
            device: device
        }
    }

    pub fn add_new(&mut self, n_mesh: &'a Mesh) {
        self.meshes.push(n_mesh);
    }

    pub fn render(&self, window: &mut GameWindow) {

    }
}