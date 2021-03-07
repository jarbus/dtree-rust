extern crate piston;
extern crate graphics;
extern crate opengl_graphics;


use glutin_window::GlutinWindow;
use piston_window::*;
use opengl_graphics::{GLGraphics, OpenGL};

use piston::input::{RenderEvent, PressEvent, Button, Key, MouseButton};
use piston::event_loop::{EventSettings, Events};


mod graph;
use graph::Graph;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GLGraphics::new(opengl);
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
                graphics::clear(color::BLACK, gl);         // clear screen
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
                Button::Mouse(MouseButton::Left) => graph.nodes.push(node::Node::new(node::Shape::Circle, cursor[0]/window_size[0], cursor[1]/window_size[1])),
                // Clear all nodes on right click
                Button::Mouse(MouseButton::Right) => graph.nodes.clear(),
                Button::Keyboard(Key::K) => graph.select(graph.selected as i8 + 1),
                Button::Keyboard(Key::J) => graph.select(graph.selected as i8 - 1),
                _ => {},
            }
        }
    }
}
