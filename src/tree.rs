use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use yoga::types::Direction;

#[derive(Debug)]
pub struct Node {
  pub name: &'static str,
  pub styles: Vec<yoga::FlexStyle>,
  pub color: Color,
  pub children: Vec<Node>,
}

impl Default for Node {
  fn default() -> Node {
    Node {
      name: "<unnamed Node>",
      styles: vec![],
      color: Color::RGB(255, 255, 255),
      children: vec![],
    }
  }
}

impl Node {
  pub fn render_root(node: Node, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
    let yoga_node = yoga::Node::new();
    let mut node_instance = NodeInstance::new(node, yoga_node);

    node_instance.build(None);
    node_instance.calculate_layout(canvas)?;
    node_instance.draw(canvas)?;

    Ok(())
  }
}

struct NodeInstance {
  config: Node,
  yoga_node: yoga::Node,
  children: Vec<NodeInstance>,
}

impl NodeInstance {
  fn new(config: Node, yoga_node: yoga::Node) -> NodeInstance {
    NodeInstance {
      config: config,
      yoga_node: yoga_node,
      children: vec![],
    }
  }

  fn build(&mut self, maybe_parent: Option<&mut yoga::Node>) {
    self.yoga_node.apply_styles(&self.config.styles);
    if let Some(parent) = maybe_parent {
      let len = parent.child_count();
      parent.insert_child(&mut self.yoga_node, len);
    }

    self.config.children.reverse();
    while let Some(child_config) = self.config.children.pop() {
      let mut child_yoga_node = yoga::Node::new();
      let mut child_instance = NodeInstance::new(child_config, child_yoga_node);
      child_instance.build(Some(&mut self.yoga_node));
      self.children.push(child_instance);
    }
  }

  fn calculate_layout(&mut self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    self
      .yoga_node
      .calculate_layout(width as f32, height as f32, Direction::LTR);

    Ok(())
  }

  fn draw(&self, canvas: &mut Canvas<sdl2::video::Window>) -> Result<(), String> {
    let layout = self.yoga_node.get_layout();
    let layout_rect = Rect::new(
      layout.left() as i32,
      layout.top() as i32,
      layout.width() as u32,
      layout.height() as u32,
    );

    canvas.set_draw_color(self.config.color);
    canvas.fill_rect(layout_rect)?;

    for child in &self.children {
      child.draw(canvas)?;
    }

    Ok(())
  }
}
