#[macro_use]
extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate sdl2;

mod display;
mod input;
mod math;
mod renderer;

use crate::display::game_window::GameWindow;
use crate::renderer::core::Core;

fn main() {
    let mut main_window = GameWindow::new("KitsuneEngine test", 800, 600);
    let mut core_renderer = Core::new();
    while !main_window.closed {
        main_window.update();
        core_renderer.render(&mut main_window)
    }
}