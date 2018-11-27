extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

mod window;
use window::Window;

pub fn main() {
  let mut window = Window::new(render, handle_event);
  window.run();
}

fn render(canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
  canvas.set_draw_color(Color::RGB(255, 255, 255));
  canvas.clear();

  canvas.set_draw_color(Color::RGB(255, 0, 0));
  let (width, height) = canvas.output_size()?;
  canvas.fill_rect(Rect::new(10, 10, width - 20, height - 20))?;

  Ok(())
}

fn handle_event(event: Event) {
  if let Event::Window { win_event, .. } = event {
    println!("win_event: {:#?}", win_event);
  } else {
    println!("event: {:#?}", event);
  }
}
