//extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

//use winit::{Event, EventsLoop, Window, WindowEvent, ControlFlow};


extern crate piston_window;
use piston_window::*;

//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

//use graphics::*;

// App data
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    pos: [f64; 2],
    window_size: [f64; 2]
}

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// App functions
impl App {
    fn render(&mut self, args: &RenderArgs) {


        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.pos[0] * args.window_size[0], self.pos[1] * args.window_size[1]) ;
        self.window_size = [args.window_size[0], args.window_size[1]];

        // |c , gl| {} is a closure, which is like an anonymous fun
        // https://doc.rust-lang.org/rust-by-example/fn/closures.html
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut change_pos = false;
    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        pos: [0.5, 0.5],
        window_size: [window.size().width, window.size().height],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                change_pos = true;

            }
        }
        if change_pos {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0], pos[1]);
                app.pos = [x / app.window_size[0], y / app.window_size[1]];
                change_pos = false;
            }
        }
    };
}

//extern crate piston_window;
//extern crate image as im;
//extern crate vecmath;
//
//use piston_window::*;
//use vecmath::*;
//
//fn main() {
//    let opengl = OpenGL::V3_2;
//    let (mut width, mut height) = (300, 300);
//    let mut window: PistonWindow =
//        WindowSettings::new("piston: paint", (width, height))
//        .exit_on_esc(true)
//        .graphics_api(opengl)
//        .build()
//        .unwrap();
//
//    let mut canvas = im::ImageBuffer::new(width, height);
//    let mut draw = false;
//    let mut texture_context = TextureContext {
//        factory: window.factory.clone(),
//        encoder: window.factory.create_command_buffer().into()
//    };
//    let mut texture: G2dTexture = Texture::from_image(
//            &mut texture_context,
//            &canvas,
//            &TextureSettings::new()
//        ).unwrap();
//
//    let mut last_pos: Option<[f64; 2]> = None;
//
//    while let Some(e) = window.next() {
//
//        //let window_size = e.render_args().window_size;
//        //width, height = window_size[0], window_size[1]
//        if let Some(args) = e.render_args() {
//            texture.update(&mut texture_context, &canvas).unwrap();
//            width = args.window_size[0] as u32;
//            height = args.window_size[1] as u32;
//            window.draw_2d(&e, |c, g, device| {
//                // Update texture before rendering.
//                texture_context.encoder.flush(device);
//
//                clear([1.0; 4], g);
//                image(&texture, c.transform, g);
//            });
//        }
//        if let Some(button) = e.press_args() {
//            if button == Button::Mouse(MouseButton::Left) {
//                draw = true;
//            }
//        };
//        if let Some(button) = e.release_args() {
//            if button == Button::Mouse(MouseButton::Left) {
//                draw = false;
//                last_pos = None
//            }
//        };
//        if draw {
//            if let Some(pos) = e.mouse_cursor_args() {
//                let (x, y) = (pos[0] as f32, pos[1] as f32);
//
//                if let Some(p) = last_pos {
//                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
//                    let distance = vec2_len(vec2_sub(p, pos)) as u32;
//
//                    for i in 0..distance {
//                        let diff_x = x - last_x;
//                        let diff_y = y - last_y;
//                        let delta = i as f32 / distance as f32;
//                        let new_x = (last_x + (diff_x * delta)) as u32;
//                        let new_y = (last_y + (diff_y * delta)) as u32;
//                        println!("putting pixel if {} < {} && {} < {}",new_x,width,new_y,height);
//                        if new_x < width && new_y < height {
//                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 255, 0, 255]));
//                        };
//                    };
//                };
//
//                last_pos = Some(pos)
//            };
//
//        }
//    }
//}
