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
}

impl Node{
    pub fn draw(&self, c: graphics::context::Context, gl: &mut GlGraphics) {
        if let Some(v) = c.viewport{
            let x0 = (self.pos[0] * v.window_size[0]) - (self.size/2.0);
            let y0 = (self.pos[1] * v.window_size[1]) - (self.size/2.0);

            match self.shape {
                Shape::Rect => graphics::Rectangle::new(self.color).draw([x0, y0,self.size, self.size], &c.draw_state, c.transform, gl),
                Shape::Circle => graphics::CircleArc::new(self.color,10.0,0.0,2.0 * std::f64::consts::PI).draw([x0,y0,self.size,self.size], &c.draw_state, c.transform, gl),
            }
        }
    }
    pub fn new(start_shape: Shape, x:f64, y:f64) -> Node {
        Node {
            shape: start_shape,
            color: WHITE,
            pos: [x, y],
            rot: 0.0,
            size: 100.0,
        }
    }
}
