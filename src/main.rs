extern crate piston;
extern crate graphics;
extern crate opengl_graphics;


use glutin_window::GlutinWindow;
use piston_window::*;

use opengl_graphics::{GlGraphics, OpenGL };

type Color = [f32; 4];

const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];


#[derive(Clone)]
enum Shape {
    Rect,
    Circle
}

#[derive(Clone)]
pub struct Node {
   shape: Shape,
   pos: [f64; 2],
   rot: f32,
   size: f64,
}

impl Node{
    fn draw(&self, c: graphics::context::Context, gl: &mut GlGraphics) {
        if let Some(v) = c.viewport{
            let x0 = (self.pos[0] * v.window_size[0]) - (self.size/2.0);
            let y0 = (self.pos[1] * v.window_size[1]) - (self.size/2.0);

            match self.shape {
                Shape::Rect => graphics::Rectangle::new(WHITE).draw([x0, y0,self.size, self.size], &c.draw_state, c.transform, gl),
                Shape::Circle => graphics::CircleArc::new(WHITE,100.0,0.0,2.0).draw([x0,y0,self.size,self.size], &c.draw_state, c.transform, gl),
            }
        }
    }
    fn new(start_shape: Shape, x:f64, y:f64) -> Node {
        Node {
            shape: start_shape,
            pos: [x, y],
            rot: 0.0,
            size: 100.0,
        }
    }
}

type Graph = Vec<Node>;

fn make_graph() -> Graph {
    let graph = vec![Node::new(Shape::Rect,0.5, 0.5); 1];
    graph
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut graph = make_graph();
    let mut events = Events::new(EventSettings::new());
    let mut cursor: [f64; 2] = [0.0, 0.0];
    let mut window_size: [f64; 2] = [0.0,0.0];
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            window_size = r.viewport().window_size;
            gl.draw(r.viewport(), |c, gl| {
                window_size = r.viewport().window_size;
                graphics::clear(BLACK, gl);
                for i in 0..graph.len() {
                    graph[i].draw(c,gl);
                }
            });
        }
        if let Some(pos) = e.mouse_cursor_args() {
            cursor = pos;
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(MouseButton::Left) => graph.push(Node::new(Shape::Rect, cursor[0]/window_size[0], cursor[1]/window_size[1])),
                Button::Mouse(MouseButton::Right) => graph.clear(),
                Button::Keyboard(Key::B) => println!("B has been pressed"),
                _ => {},
            }
        }
    }
}
