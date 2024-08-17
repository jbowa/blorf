use std::sync::Arc;

use wasm_bindgen::{prelude::wasm_bindgen, UnwrapThrowExt};
use web_sys::Element;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    platform::web::WindowExtWebSys,
    window::{Window, WindowId},
};

mod utils;

pub const CANVAS_ID: &str = "blorf";

struct State {}

impl State {
    async fn new(window: Arc<Window>) -> State {
        log::info!("------ state ------");
        Self {}
    }
}

enum UserEvent {
    StateReady(State),
}
struct App {
    event_loop_proxy: EventLoopProxy<UserEvent>,
}

impl App {
    fn new(event_loop: &EventLoop<UserEvent>) -> Self {
        Self {
            event_loop_proxy: event_loop.create_proxy(),
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("window resumed");
        let window_attrs = Window::default_attributes();
        let window = event_loop
            .create_window(window_attrs)
            .expect("could not create window");

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = window.canvas().expect("window should have a canvas");
                canvas.set_id(CANVAS_ID);
                canvas
                    .style()
                    .set_property("background-color", "coral")
                    .expect("could not set canvas background");
                let canvas_element = Element::from(canvas);

                doc.body()
                    .expect("document should have a body")
                    .append_child(&canvas_element)
                    .expect("could not append canvas to body");
                Some(())
            })
            .expect("could not handle canvas element");

        let _ = window.request_inner_size(PhysicalSize::new(500, 500));

        let state_future = State::new(Arc::new(window));
        let event_loop_proxy = self.event_loop_proxy.clone();
        let future = async move {
            let state = state_future.await;
            assert!(event_loop_proxy
                .send_event(UserEvent::StateReady(state))
                .is_ok());
        };
        wasm_bindgen_futures::spawn_local(future)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        log::info!("----- window event with ID {:?} ------", window_id);
        // TODO:
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        log::info!("----- waiting ------");
        // TODO:
    }
}

pub async fn run() {
    utils::gpu_info::adapter_info().await;

    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .unwrap_throw();
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap_throw();
}

#[wasm_bindgen]
pub fn web() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    log::info!("launching…");

    wasm_bindgen_futures::spawn_local(run());
}
