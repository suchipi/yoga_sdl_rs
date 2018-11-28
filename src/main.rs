extern crate sdl2;
#[macro_use]
extern crate yoga;
mod tree;
mod window;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use tree::Node;
use window::Window;
use yoga::prelude::*;

pub fn main() {
  let mut window = Window::new(render, handle_event);
  window.run();
}

fn render(canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
  canvas.set_draw_color(Color::RGB(255, 255, 255));
  canvas.clear();

  Node::render_root(
    Node {
      styles: make_styles![
        Top(0 pt),
        Right(0 pt),
        Bottom(0 pt),
        Left(0 pt),
        FlexDirection(yoga::types::FlexDirection::Column)
      ],
      children: vec![
        Node {
          name: "header (red)",
          color: Color::RGB(255, 0, 0),
          styles: make_styles![
            FlexBasis(200 pt),
            AlignItems(yoga::types::Align::Center),
            JustifyContent(yoga::types::Justify::Center)
          ],
          children: vec![Node {
            name: "hamburger menu",
            color: Color::RGB(255, 255, 255),
            styles: make_styles![
              Width(10 pt),
              Height(10 pt)
            ],
            ..Default::default()
          }],
          ..Default::default()
        },
        Node {
          name: "content (green)",
          color: Color::RGB(0, 255, 0),
          styles: make_styles![FlexGrow(1.0)],
          ..Default::default()
        },
        Node {
          name: "footer (blue)",
          color: Color::RGB(0, 0, 255),
          styles: make_styles![
            FlexBasis(100 pt)
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    },
    canvas,
  )
}

fn handle_event(event: Event) {
  if let Event::Window { win_event, .. } = event {
    println!("win_event: {:#?}", win_event);
  } else {
    println!("event: {:#?}", event);
  }
}
