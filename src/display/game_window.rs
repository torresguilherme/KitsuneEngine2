use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::{Window};
use sdl2::render::{Canvas};
use std::time::Duration;

pub struct GameWindow {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub closed: bool
}

impl GameWindow {
    pub fn new(name: &str, width: u32, height: u32) -> GameWindow {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(name, width, height)
            .position_centered().build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();

        GameWindow {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            canvas: canvas,
            event_pump: event_pump,
            closed: false
        }
    }

    pub fn update(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { self.closed = true },
                _ => {}
            }
        }

        self.canvas.present();
    }
}