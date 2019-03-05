extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate sdl2;

mod display;

use crate::display::game_window::GameWindow;

fn main() {
    let mut main_window = GameWindow::new("KitsuneEngine test", 800, 600);
    main_window.update();
}