#[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsCast, UnwrapThrowExt};
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

pub async fn run() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    let adapter = {
        let instance = wgpu::Instance::default();
        #[cfg(not(target_arch = "wasm32"))]
        {
            log::info!("Available adapters:");
            for a in instance.enumerate_adapters(wgpu::Backends::all()) {
                log::info!("    {:?}", a.get_info())
            }
        }
        instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap()
    };

    log::info!("Selected adapter: {:?}", adapter.get_info());

    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = Application::new();
    event_loop.run_app(&mut app).unwrap_throw();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native() {
    env_logger::builder()
        .filter(Some(module_path!()), log::LevelFilter::Info)
        .parse_default_env()
        .init();

    pollster::block_on(run());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn web() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("Could not initialize logger");

    log::info!("Initiating wasm");

    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let canvas = document.create_element("canvas").unwrap_throw();
    canvas.set_id(CANVAS_ID);
    canvas.set_attribute("width", "250").unwrap_throw();
    canvas.set_attribute("height", "250").unwrap_throw();
    canvas
        .set_attribute("style", "background-color: green;")
        .unwrap_throw();

    let body = document.body().unwrap_throw();
    body.append_child(&canvas.unchecked_ref()).unwrap_throw();

    wasm_bindgen_futures::spawn_local(run());
}
