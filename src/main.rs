#[macro_use]
extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate sdl2;
extern crate winit;

mod display;
mod input;
mod math;
mod renderer;

use crate::renderer::core::Core;
use crate::renderer::mesh::Mesh;
use crate::math::vec3::Vec3;

fn main() {
    let mut core_renderer = Core::new("KitsuneEngine test", 800, 600);
    let triangle = Mesh::new(
        vec![Vec3{x:-0.5, y:-0.5, z:0.0}, Vec3{x:0.0, y:0.5, z:0.0}, Vec3{x:0.5, y:-0.25, z:0.0}],
        vec![0],
        vec![],
        vec![],
        vec![],
        vec![]
    );
    core_renderer.add_new(&triangle);
    core_renderer.create_command_buffers();
    while !core_renderer.done {
        core_renderer.render();
    }
}