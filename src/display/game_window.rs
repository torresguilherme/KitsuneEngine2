use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::{Window};
use sdl2::render::{Canvas};
use std::time::Duration;

pub struct GameWindow {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    canvas: Canvas<Window>,
    event_pump: EventPump
}