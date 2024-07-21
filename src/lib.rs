use wasm_bindgen::prelude::*;
use winit::{application::ApplicationHandler, event, event_loop::EventLoop};

pub const CANVAS_ID: &str = "blorf";

struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {
            // TODO:
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // TODO:
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: event::WindowEvent,
    ) {
        // TODO:
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // TODO:
    }
}

pub fn run() {
    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = Application::new();
    event_loop.run_app(&mut app).unwrap_throw();
}

#[wasm_bindgen]
pub fn run_web() {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let canvas = document.create_element("canvas").unwrap_throw();
    canvas.set_id(CANVAS_ID);
    canvas.set_attribute("width", "250").unwrap_throw();
    canvas.set_attribute("height", "250").unwrap_throw();
    canvas
        .set_attribute("style", "background-color: blue;")
        .unwrap_throw();

    let body = document.body().unwrap_throw();
    body.append_child(&canvas.unchecked_ref()).unwrap_throw();

    run();
}
