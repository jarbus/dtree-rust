extern crate piston;
extern crate graphics;
extern crate opengl_graphics;


use glutin_window::GlutinWindow;
use piston::WindowSettings;


use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, PressEvent, Button, ButtonState, Key, MouseButton};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::{ButtonEvent, RenderEvent};

use opengl_graphics::{GlGraphics, OpenGL };

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let graph = make_graph();
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                graphics::clear(BLACK, gl);
                for i in 0..graph.len() {
                    graph[i].draw(c,gl);
                }
            });
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {

            }
            else if button == Button::Keyboard(Key::B){
                println!("B has been pressed");
            }
        }
    }
}
