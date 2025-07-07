pub mod model_instance;

use model_instance::{Instance, RawInstance};
use nalgebra::{Matrix4, Point3, Vector3};
use std::sync::Arc;
use wgpu::util::DeviceExt;

use wgpu::{
    BindGroup, Buffer, Device, DeviceDescriptor, Queue, RenderPipeline, Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

use crate::camera::Camera;
use crate::camera::camera_uniform::CameraUniform;
use crate::camera::light_uniform::PointLightUniform;
use crate::model::Model;
use crate::model::depth_texture::DepthTexture;
use crate::model::maps::map_1::Map1;
use crate::model::texture::Texture;
use crate::model::vertex::Vertex;

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    models: Vec<Model>,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    point_light_bind_group: BindGroup,
    depth_texture: DepthTexture,
    is_surface_configured: bool,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let size = window.inner_size();

        let surface = instance
            .create_surface(window.clone())
            .map_err(|_| "Failed to create surface")?;

        let adaptor = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| "Failed to request Adapter")?;

        let (device, queue) = adaptor
            .request_device(&DeviceDescriptor::default())
            .await
            .map_err(|_| "Failed to request device")?;

        let surface_caps = surface.get_capabilities(&adaptor);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        let camera = Camera {
            position: Point3::new(0.0, 0.5, 1.0),
            target: Point3::origin(),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: size.width as f32 / size.height as f32,
            fovy: 1.0,
            near: 0.1,
            far: 200.0,
            is_w_pressed: false,
            is_s_pressed: false,
            is_a_pressed: false,
            is_d_pressed: false,
            yaw: 0.0,
            pitch: 0.0,
            delta: None,
        };

        let mut camera_uniform = CameraUniform::new(camera.position);
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout = CameraUniform::create_bind_group_layout(&device);

        let camera_bind_group =
            CameraUniform::create_bind_group(&device, &camera_bind_group_layout, &camera_buffer);

        let point_light_uniform = PointLightUniform::new(Point3::new(4.0, 0.3, 0.5), 1.0);
        let point_light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Light Buffer"),
            contents: bytemuck::cast_slice(&[point_light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let point_light_bind_group_layout = PointLightUniform::create_bind_group_layout(&device);
        let point_light_bind_group = PointLightUniform::create_bind_group(
            &device,
            &point_light_bind_group_layout,
            &point_light_buffer,
        );

        let diffuse_texture_layout = Texture::create_bind_group_layout(&device);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &point_light_bind_group_layout,
                    &diffuse_texture_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), RawInstance::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let depth_texture = DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        let models = Map1::get_models(&device, &queue, &diffuse_texture_layout);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: true,
            models,
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            point_light_bind_group,
            depth_texture,
            render_pipeline,
        })
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }
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
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.point_light_bind_group, &[]);
            for model in &self.models {
                model.draw(&mut render_pass);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update(&mut self) {
        self.camera.update_camera(0.05, 0.05, 0.004);
        self.camera_uniform.update_view_proj(&self.camera);
        self.camera_uniform.update_camera_pos(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.depth_texture =
                DepthTexture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }
}
