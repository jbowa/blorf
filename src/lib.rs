use std::{borrow::Cow, sync::Arc};

use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use wasm_bindgen::{prelude::wasm_bindgen, throw_str, UnwrapThrowExt};
use web_sys::Element;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    platform::web::WindowExtWebSys,
    window::{Window, WindowId},
};

mod utils;

pub const CANVAS_ID: &str = "blorf";

struct State {
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,
    state_ready: bool,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let mut size = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let instance_desc = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_desc);

        let surface = instance
            .create_surface(window.clone())
            .unwrap_or_else(|e| throw_str(&format!("{e:#?}")));

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Create the logical device and queue command.
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter,
                    // so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);

        let state_ready = false;
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        // Load the shaders from disk.
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(surface_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            config,
            device,
            queue,
            size,
            surface,
            window,
            state_ready,
            render_pipeline,
        }
    }

    fn input(&mut self, _: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let clear_color = wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            };
            let color_attachment = wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: wgpu::StoreOp::Store,
                },
            };
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

enum UserEvent {
    StateReady(State),
}
struct App {
    state: Option<State>,
    event_loop_proxy: EventLoopProxy<UserEvent>,
}

impl App {
    fn new(event_loop: &EventLoop<UserEvent>) -> Self {
        Self {
            state: None,
            event_loop_proxy: event_loop.create_proxy(),
        }
    }
}

impl winit::application::ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes();
        let window = event_loop
            .create_window(window_attrs)
            .expect("failed to create window");

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let canvas = window.canvas().expect("window should have a canvas");
                canvas.set_id(CANVAS_ID);
                let canvas_element = Element::from(canvas);

                doc.body()
                    .expect("document should have a body")
                    .append_child(&canvas_element)
                    .expect("could not append canvas to body");
                Some(())
            })
            .expect("could not handle canvas element");

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

    // Emitted when an event is sent from EventLoopProxy::send_event.
    fn user_event(&mut self, _: &ActiveEventLoop, event: UserEvent) {
        let UserEvent::StateReady(state) = event;
        self.state = Some(state);
    }

    // Emitted when the OS sends an event to a winit window.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(ref mut state) = self.state else {
            return;
        };

        if window_id != state.window.id() {
            tracing::warn!("window ID mismatch!")
        }

        if state.input(&event) {
            return;
        }

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                tracing::info!("Exited!");
                event_loop.exit()
            }
            WindowEvent::Resized(new_size) => {
                state.state_ready = true;
                state.resize(new_size);
            }
            WindowEvent::RedrawRequested => {
                if !state.state_ready {
                    tracing::warn!("State was not ready!");
                    return;
                }

                state.update();

                match state.render() {
                    Ok(()) => {}
                    // A timeout was encountered while trying to acquire the
                    // next frame. This happens when a frame takes too long to
                    // present.
                    Err(wgpu::SurfaceError::Timeout) => {
                        tracing::warn!("Surface timeout!")
                    }
                    // Reconfigure the surface if it is lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size);
                    }
                    // The system is out of memory. We should gracefully exit
                    // the program.
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        tracing::error!("System out of memory!");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    // Emitted when the event loop is about to block and wait for new events.
    // NOTE: We should‘nt call window_redraw here as it may lead to unecessary
    // re-renders.
    // fn about_to_wait(&mut self, _: &ActiveEventLoop) {}

    // Emitted when the application has been suspended.
    // fn suspended(&mut self, event_loop: &ActiveEventLoop) {}
}

pub async fn run() {
    _ = utils::gpu_info::adapter_info().await;

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    let mut app = App::new(&event_loop);
    event_loop.run_app(&mut app).unwrap_throw();
}

#[wasm_bindgen]
pub fn web() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy()
        .add_directive("wgpu_core::device::resource=warn".parse().unwrap());
    let subscriber = tracing_subscriber::registry().with(env_filter);

    {
        use tracing_wasm::{WASMLayer, WASMLayerConfig};
        console_error_panic_hook::set_once();

        let wasm_layer = WASMLayer::new(WASMLayerConfig::default());
        subscriber.with(wasm_layer).try_init().unwrap();
    }

    wasm_bindgen_futures::spawn_local(run());
}
