use std::iter;
use wgpu::util::DeviceExt;
use cgmath::prelude::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

mod camera;
mod model;
mod pixel;
mod graphics;

//const PIXELAMOUNT: usize = 1600;
//const TOTALPIXELS: usize = PIXELAMOUNT*PIXELAMOUNT;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    camera: camera::Camera,
    projection: camera::Projection,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: camera::CameraController,
    model: model::Model,
    model_buffer: wgpu::Buffer,
    model_bind_group: wgpu::BindGroup,
    time: model::Instant,
    mouse_pressed: bool,
    full_screen: bool,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let (adapter, device, queue) = graphics::create_device(&instance, &surface).await;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let camera = camera::Camera::new((0.0, 10.0, 20.0), cgmath::Deg(-90.0), cgmath::Deg(0.0));
        let projection = camera::Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);
        let camera_controller = camera::CameraController::new(10.0, 0.4);

        let camera_buffer = graphics::create_camera_buffer(&device, camera_uniform);
        let camera_bind_group_layout = graphics::create_camera_bind_group_layout(&device);
        let camera_bind_group = graphics::create_camera_bind_group(&device, &camera_buffer, &camera_bind_group_layout);

        let time = model::Instant::now();
        let mut model = model::Model::new();
        model.update_current_time(time);
        model.update_model();

        let model_buffer = graphics::create_model_buffer(&device, model);
        let model_bind_group_layout = graphics::create_model_bind_group_layout(&device);
        let model_bind_group = graphics::create_model_bind_group(&device, &model_buffer, &model_bind_group_layout);

        let pixels = State::update_pixel_count(size.width as usize, size.height as usize);

        let vertex_buffer = graphics::create_vertex_buffer(&device, &pixels);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout = graphics::create_pipeline_layout(&device, &camera_bind_group_layout, &model_bind_group_layout);
        let render_pipeline = graphics::create_render_pipeline(&device, &render_pipeline_layout, &shader, &config);

        Self {
            surface,
            device,
            queue,
            size,
            config,
            render_pipeline,
            vertex_buffer,
            camera,
            projection,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            model,
            model_buffer,
            model_bind_group,
            time,
            mouse_pressed: false,
            full_screen: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            
            let pixels = State::update_pixel_count(new_size.width as usize, new_size.height as usize);

            self.vertex_buffer = self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&pixels),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );
        }
    }

    fn update_pixel_count(width: usize, height: usize) -> Vec<pixel::Pixel> {
        let mut pixels: Vec<pixel::Pixel> = Vec::with_capacity(width*height);
        for x in 0..width {
            for y in 0..height {
                pixels.push(pixel::Pixel{ position: [(x as i32-(width/2) as i32) as f32 / (width/2) as f32, (y as i32-(height/2) as i32) as f32 / (height/2) as f32, 0.0] });
            }
        }   
        return pixels;
    }

    #[allow(unused_variables)]
    fn input(&mut self, window: &Window, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => {
                match key {
                    VirtualKeyCode::Escape => {
                        self.full_screen = false;
                    }
                    _ => {},
                }
                self.camera_controller.process_keyboard(*key, *state)
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                let prev = self.mouse_pressed;
                self.mouse_pressed = *state == ElementState::Pressed;
                if prev != self.mouse_pressed {
                    _ = window.set_cursor_grab(true);
                }
                if !self.full_screen {
                    #[cfg(target_arch="wasm32")] {
                    window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    }
                    self.full_screen = true;
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, dt: std::time::Duration) {
        let old_time = self.time;
        self.model.update_current_time(self.time);
        self.model.update_model();
        self.queue.write_buffer(
            &self.model_buffer,
            0,
            bytemuck::cast_slice(&[self.model]),
        );

        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.model_bind_group, &[]);
            render_pass.draw(0..((self.size.width * self.size.height)-1) as u32, 0..1);
            //render_pass.draw(0..((1600*1600)-1) as u32, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(2400, 1600));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }
    
    let mut state = State::new(&window).await;
    let mut last_render_time = instant::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } => {
                state.camera_controller.process_mouse(delta.0, delta.1)
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(&window, event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
