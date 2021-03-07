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

const DEFAULT_NODE_COLOR: Color = WHITE;
const SELECTED_NODE_COLOR: Color = RED;


#[derive(Clone)]
enum Shape {
    Rect,
    Circle
}

#[derive(Clone)]
pub struct Node {
   shape: Shape,
   color: Color,
   pos: [f64; 2],
   rot: f32,
   size: f64,
}

#[derive(Clone)]
struct Graph {
    nodes: Vec<Node>,
    selected: usize,
}

impl Node{
    fn draw(&self, c: graphics::context::Context, gl: &mut GlGraphics) {
        if let Some(v) = c.viewport{
            let x0 = (self.pos[0] * v.window_size[0]) - (self.size/2.0);
            let y0 = (self.pos[1] * v.window_size[1]) - (self.size/2.0);

            match self.shape {
                Shape::Rect => graphics::Rectangle::new(self.color).draw([x0, y0,self.size, self.size], &c.draw_state, c.transform, gl),
                Shape::Circle => graphics::CircleArc::new(self.color,10.0,0.0,2.0 * std::f64::consts::PI).draw([x0,y0,self.size,self.size], &c.draw_state, c.transform, gl),
            }
        }
    }
    fn new(start_shape: Shape, x:f64, y:f64) -> Node {
        Node {
            shape: start_shape,
            color: WHITE,
            pos: [x, y],
            rot: 0.0,
            size: 100.0,
        }
    }
}


impl Graph{
    fn new() -> Graph {
        Graph { nodes: vec![Node::new(Shape::Circle,0.5, 0.5); 1], selected:0}
    }
    // Function to change selected node to the node at the new_selection position in the self.nodes
    fn select(&mut self, new_selection: i8){
        // Here we handle cases where new_selection refers to an index outside of self.nodes
        // by having the index loop back around to the opposite side of the array
        let new_selection: usize = match new_selection {
            // If goes past the last node in the vector, wrap it to the first
            x if x >= self.nodes.len() as i8 => 0,
            // If it goes below 0, wrap it to the last node in the vector
            x if x < 0 => self.nodes.len() - 1,
            _ => new_selection as usize,
            };

        // Unselect previous node, select new selection
        self.nodes[self.selected as usize].color = DEFAULT_NODE_COLOR;
        self.nodes[new_selection as usize].color = SELECTED_NODE_COLOR;
        self.selected = new_selection;
    }
}



fn main() {

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("dtree", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut graph: Graph = Graph::new();
    let mut events = Events::new(EventSettings::new());
    let mut cursor: [f64; 2] = [0.0, 0.0];          // cursor position in pixel coordinates
    let mut window_size: [f64; 2] = [0.0,0.0];      // Resolution, updated each tick
    // Steps through each type of window event
    while let Some(e) = events.next(&mut window) {
        // Draws screen on render event
        if let Some(r) = e.render_args() {
            window_size = r.viewport().window_size;
            gl.draw(r.viewport(), |c, gl| {
                graphics::clear(BLACK, gl);         // clear screen
                for i in 0..graph.nodes.len() {     // draw each node
                    graph.nodes[i].draw(c,gl);
                }
            });
        }

        if let Some(resize_event) = e.resize_args() { // update window_size every
            window_size = resize_event.window_size;   // time it's changed
        }
        if let Some(pos) = e.mouse_cursor_args() {
            cursor = pos;                             // get new cursor position each tick
        }
        if let Some(button) = e.press_args() {
            match button {
                // Add new node to graph on left click at cursor position
                Button::Mouse(MouseButton::Left) => graph.nodes.push(Node::new(Shape::Circle, cursor[0]/window_size[0], cursor[1]/window_size[1])),
                // Clear all nodes on right click
                Button::Mouse(MouseButton::Right) => graph.nodes.clear(),
                Button::Keyboard(Key::K) => graph.select(graph.selected as i8 + 1),
                Button::Keyboard(Key::J) => graph.select(graph.selected as i8 - 1),
                _ => {},
            }
        }
    }
}
