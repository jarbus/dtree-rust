#[derive(Clone)]
pub enum Shape {
    Rect,
    Circle
}

#[derive(Clone)]
pub struct Node {
   pub shape: Shape,
   pub color: Color,
   pub pos: [f64; 2],
   pub rot: f32,
   pub size: f64,
   pub id: usize,
   pub parent: usize,
   pub children: Vec<usize>,
}

impl Node{
    pub fn draw(&self, c: graphics::context::Context, gl: &mut GlGraphics, position: [f64; 2]) {
        if let Some(v) = c.viewport{
            let x0 = ((0.5 + (position[0] - self.pos[0])) * v.window_size[0]) - (self.size/2.0);
            let y0 = ((0.5 + (position[1] - self.pos[1])) * v.window_size[1]) - (self.size/2.0);

            match self.shape {
                Shape::Rect => graphics::Rectangle::new(self.color).draw([x0, y0,self.size, self.size], &c.draw_state, c.transform, gl),
                Shape::Circle => graphics::CircleArc::new(self.color,10.0,0.0,2.0 * std::f64::consts::PI).draw([x0,y0,self.size,self.size], &c.draw_state, c.transform, gl),
            }
        }
    }
    pub fn new(start_shape: Shape, x:f64, y:f64, node_id: usize, parent_node: usize) -> Node {
        Node {
            shape: start_shape,
            color: WHITE,
            pos: [x, y],
            rot: 0.0,
            size: 100.0,
            id: node_id,
            parent: parent_node,
            children: vec![],
        }
    }
}
