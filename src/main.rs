extern crate piston;
extern crate graphics;
extern crate find_folder;

mod mode;
mod view;
mod color;
mod renderer;
mod graph;
mod node;

use piston_window::*;

use piston::input::{RenderEvent, PressEvent, Button, Key, MouseButton};
use piston::event_loop::{EventSettings, Events};




fn main() {

    //let opengl = OpenGL::V3_2;
    //let gl = GlGraphics::new(opengl);
    let settings = WindowSettings::new("dtree", [512; 2]).exit_on_esc(true);
    let mut window: PistonWindow = settings.build().expect("Could not create window");
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let mut graph: graph::Graph = graph::Graph::new();
    let mut events = Events::new(EventSettings::new());
    // Steps through each type of window event
    let view = view::View::ThreeGen;
    while let Some(e) = events.next(&mut window) {

        // draw nodes
        window.draw_2d(&e, |c, g, device| {
          graphics::clear(color::BLACK, g);    // clear screen
          let mut renderer = renderer::Renderer::new(c, g, &mut glyphs, &view);
          graph.draw_view(&mut renderer);      // render graph
          glyphs.factory.encoder.flush(device);
        });

        //this will print out each character on a new line
        e.text(|text| println!("Typed '{}'", text));

        if let Some(button) = e.press_args() {

            match button {

                // Add new node to graph on left click at cursor position
                Button::Keyboard(Key::O) => graph.add_child(),
                // Clear all nodes on right click
                Button::Mouse(MouseButton::Right) => graph.reset(),
                // Select Parent Node
                Button::Keyboard(Key::K) => graph.select(-1),
                // Select left child
                Button::Keyboard(Key::H) => graph.select(1),
                // Select right child
                Button::Keyboard(Key::L) => graph.select(2),

                Button::Keyboard(Key::T) => graph.toggle_travel_mode(),
                Button::Keyboard(Key::E) => graph.edit_node(),
                Button::Keyboard(Key::Q) => break,
                _ => {},
            }
        }
    }
}
