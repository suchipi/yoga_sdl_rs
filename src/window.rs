extern crate sdl2;

use sdl2::event::*;
use std::time::Duration;

pub struct Window {
  pub render: fn(&mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>,
  pub handle_event: fn(Event),
  sdl_context: sdl2::Sdl,
  canvas: sdl2::render::Canvas<sdl2::video::Window>,
  width: u32,
  height: u32,
}

impl Window {
  pub fn new(
    render: fn(&mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String>,
    handle_event: fn(Event),
  ) -> Window {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
      .window("SDL2 Window", 800, 600)
      .position_centered()
      .opengl()
      .resizable()
      .build()
      .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();

    Window {
      render: render,
      handle_event: handle_event,
      sdl_context: sdl_context,
      canvas: canvas,
      width: 800,
      height: 600,
    }
  }

  fn _render(&mut self) {
    (self.render)(&mut self.canvas).unwrap();
    self.canvas.present();
  }

  pub fn run(&mut self) {
    self._render();

    let mut event_pump = self.sdl_context.event_pump().unwrap();
    let fps = 60;

    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => {
            (self.handle_event)(event);
            break 'running;
          }
          Event::Window { win_event, .. } => match win_event {
            WindowEvent::SizeChanged(x, y) => {
              self.width = x as u32;
              self.height = y as u32;
              (self.handle_event)(event);
            }
            _ => {
              (self.handle_event)(event);
            }
          },
          _ => {
            (self.handle_event)(event);
          }
        }
      }

      self._render();

      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }
  }
}
