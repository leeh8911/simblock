use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use std::process;

pub struct Application {
    window: GlutinWindow,
    gl: GlGraphics,
}

impl Application {
    pub fn new(name: &str, size: [u32; 2]) -> Application {
        let opengl = OpenGL::V3_2;
        let mut window: GlutinWindow = WindowSettings::new(name, size)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Application {
            window: window,
            gl: GlGraphics::new(opengl),
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                println!("Rendered");
            }
            if let Some(args) = e.update_args() {
                println!("Updated");
            }
            if let Some(button) = e.press_args() {
                println!("Pressed: {:?}", button);
            }
            if let Some(button) = e.release_args() {
                println!("Released: {:?}", button);
            }
        }
    }
}
