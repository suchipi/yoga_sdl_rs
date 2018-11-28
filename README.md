# yoga_sdl_rs

An experiment using [Yoga](https://github.com/bschwind/yoga-rs) to layout rectangles in a resizable [SDL2](https://github.com/Rust-SDL2/rust-sdl2) window, in Rust.

```rs
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
            AlignItems(yoga::types::Align::FlexStart),
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
```

If you think this is cool, you may also like [Azul](https://azul.rs/).

## LICENSE

MIT
