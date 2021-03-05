//! A Roguelike Game using Piston Engine

extern crate piston;

extern crate graphics;
extern crate opengl_graphics;


#[macro_use]
extern crate sum_type;

use glutin_window::GlutinWindow;
use piston::WindowSettings;



use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::RenderEvent;

use opengl_graphics::{GlGraphics, OpenGL};

//use types::{Rectangle, Ellipse};

type Color = [f64; 4];

const RED: Color = [1.0, 0.0, 0.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;



pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    window_size: [f64; 2],
    nodes: Vec<Node>,
}

sum_type!{
    #[derive(Clone)]
    pub enum Shape {
        Rectangle(graphics::rectangle::Rectangle),
        Circle(graphics::circle_arc::CircleArc),
    }
}


#[derive(Clone)]
pub struct Node {
   shape: Shape,
   pos: [f64; 2],
   rot: f64,
}


pub fn new_rec() -> Node {
    Node {
        shape: Shape::Rectangle(graphics::Rectangle::new(WHITE)) ,
        pos: [0.0, 0.0],
        rot: 0.0,
    }
}
pub fn new_circle() -> Node {
    Node {
        shape: Shape::Circle(graphics::CircleArc::new(WHITE,100.0,0.0,2.0)),
        pos: [0.0, 0.0],
        rot: 0.0
    }
}


type Map = Vec<Vec<Node>>;

fn make_map() -> Map {
    let mut map = vec![vec![new_rec(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Roguelike", [512; 2]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
    let map = make_map();
    let nodes : Vec<Node> = vec![Node{s}];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                graphics::clear(BLUE, g);

                for i in 0..WORLD_SIZE {
                    for j in 0..WORLD_SIZE {
                        let pos: [f64; 4] = [
                            PIXEL_SIZE * i as f64,
                            PIXEL_SIZE * j as f64,
                            PIXEL_SIZE * (i + 1) as f64,
                            PIXEL_SIZE * (j + 1) as f64,
                        ];
                        graphics::Rectangle::new(map[i as usize][j as usize].color).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }
    }
}
