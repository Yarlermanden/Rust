use bytemuck::NoUninit;
use wgpu::{Device, BindGroupLayout, Buffer, BindGroup, ShaderModule, PipelineLayout, RenderPipeline, SurfaceConfiguration, Queue, Instance, Surface, Adapter};
use wgpu::util::DeviceExt;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::camera::CameraUniform;
use crate::model::Model;
use crate::pixel::Pixel;


pub fn create_camera_buffer(device: &Device, camera_uniform: CameraUniform) -> Buffer {
    create_buffer(device, camera_uniform, "Camera Buffer")
}

pub fn create_camera_bind_group_layout(device: &Device) -> BindGroupLayout {
    create_bind_group_layout(device, "camera_bind_group_layout")
}

pub fn create_camera_bind_group(device: &Device, buffer: &Buffer, layout: &BindGroupLayout) -> BindGroup
{
    create_bind_group(device, buffer, layout, "camera_bind_group")
}

pub fn create_model_buffer(device: &Device, model: Model) -> Buffer {
    create_buffer(device, model, "Model Buffer")
}

pub fn create_model_bind_group_layout(device: &Device) -> BindGroupLayout {
    create_bind_group_layout(device, "model_bind_group_layout")
}

pub fn create_model_bind_group(device: &Device, buffer: &Buffer, layout: &BindGroupLayout) -> BindGroup
{
    create_bind_group(device, buffer, layout, "model_bind_group")
}

fn create_buffer<T: ?Sized>(device: &Device, uniform: T, label: &str) -> Buffer 
where T: NoUninit
{
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(&[uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

fn create_bind_group_layout(device: &Device, label: &str) -> BindGroupLayout 
{
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: Some(label),
    })
}

fn create_bind_group(device: &Device, buffer: &Buffer, layout: &BindGroupLayout, label: &str) -> BindGroup 
{
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: Some(label),
    })
}

pub fn create_vertex_buffer<T>(device: &Device, pixels: &[T]) -> Buffer
where T: NoUninit
{
    device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(pixels),
            usage: wgpu::BufferUsages::VERTEX,
        }
    )
}



pub fn create_pipeline_layout(device: &Device, camera_layout: &BindGroupLayout, model_layout: &BindGroupLayout) -> PipelineLayout
{
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            &camera_layout,
            &model_layout,
        ],
        push_constant_ranges: &[],
    })
}

pub fn create_render_pipeline(device: &Device, layout: &PipelineLayout, shader: &ShaderModule, config: &SurfaceConfiguration) -> RenderPipeline
{
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                Pixel::desc(),
            ],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}



pub async fn create_device(instance: &Instance, surface: &Surface) -> (Adapter, Device, Queue)
{
    let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .unwrap();
        return (adapter, device, queue);
}