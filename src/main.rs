// TODO turn node vector into hashmap and change traversal to go between children and parents
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use std::collections::HashMap;
use glutin_window::GlutinWindow;
use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderEvent, PressEvent, Button, Key, MouseButton};
use piston::event_loop::{EventSettings, Events};

include!("color.rs");
include!("node.rs");
include!("graph.rs");


fn main() {

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("dtree", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let mut graph: Graph = Graph::new();
    let mut events = Events::new(EventSettings::new());
    //let mut cursor: [f64; 2] = [0.0, 0.0];            // cursor position in pixel coordinates
    //let mut window_size: [f64; 2] = [0.0,0.0];        // Resolution, updated each tick
    // Steps through each type of window event
    while let Some(e) = events.next(&mut window) {
        // Draws screen on render event
        if let Some(r) = e.render_args() {
            //window_size = r.viewport().window_size;
            gl.draw(r.viewport(), |c, gl| {
                graphics::clear(color::BLACK, gl);    // clear screen
                graph.draw(c,gl);                     // render graph
            });
        }
        //if let Some(resize_event) = e.resize_args() { // update window_size every
        //    window_size = resize_event.window_size;   // time it's changed
        //}
        //if let Some(pos) = e.mouse_cursor_args() {
        //    cursor = pos;                             // get new cursor position each tick
        //}

        if let Some(button) = e.press_args() {

            match button {
                // Add new node to graph on left click at cursor position
                Button::Keyboard(Key::O) => graph.add_child(),
                // Clear all nodes on right click
                Button::Mouse(MouseButton::Right) => graph.reset(),
                Button::Keyboard(Key::K) => graph.select(-1),
                Button::Keyboard(Key::H) => graph.select(1),
                Button::Keyboard(Key::L) => graph.select(2),
                _ => {},
            }
        }
    }
}
