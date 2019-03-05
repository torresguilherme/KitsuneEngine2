extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate sdl2;

mod math;
mod display;

use crate::math::vec2::Vec2;
use crate::display::game_window::GameWindow;

fn main() {
    let mut result = Vec2{x:3.0, y:4.0};
    println!("{}", result.lenght());
    println!("{}", result.normalized().lenght());
    println!("{}", result.rotate(32.0).lenght());
}