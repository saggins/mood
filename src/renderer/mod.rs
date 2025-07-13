use nalgebra::{Point3, Vector3};
use pipeline_factory::PipelineFactory;
use std::sync::Arc;
use wgpu::util::{DeviceExt, RenderEncoder};

use wgpu::{
    BindGroup, Buffer, Device, DeviceDescriptor, Queue, RenderPipeline, Surface,
    SurfaceConfiguration, VertexBufferLayout,
};
use winit::window::Window;

use crate::camera::Camera;
use crate::camera::camera_uniform::CameraUniform;
use crate::camera::light::Light;
use crate::camera::light_uniform::LightUniformArray;
use crate::collision::collision_manager;
use crate::model::Model;
use crate::model::cube_texture::{CubeTexture, CubeTextureBuilder};
use crate::model::depth_texture::DepthTexture;
use crate::model::maps::map_1::Map1;
use crate::model::model_instance::RawInstance;
use crate::model::texture::TextureBuilder;
use crate::model::vertex::Vertex;

mod pipeline_factory;

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
    skybox_bind_group: BindGroup,
    skybox_render_pipeline: RenderPipeline,
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

        // layouts
        let camera_bind_group_layout = CameraUniform::create_bind_group_layout(&device);
        let diffuse_texture_layout = TextureBuilder::create_bind_group_layout(&device);
        let point_light_bind_group_layout = LightUniformArray::create_bind_group_layout(&device);
        let skybox_bind_group_layout = CubeTextureBuilder::create_bind_group_layout(&device);
        let render_pipeline_layout = PipelineFactory::create_render_pipeline_layout(
            &device,
            &[
                &camera_bind_group_layout,
                &point_light_bind_group_layout,
                &diffuse_texture_layout,
            ],
        );
        let skybox_pipeline_layout = PipelineFactory::create_render_pipeline_layout(
            &device,
            &[&skybox_bind_group_layout, &camera_bind_group_layout],
        );

        let (models, skybox_files, lights, collision_manager) =
            Map1::get_models(&device, &queue, &diffuse_texture_layout);
        let camera = Camera {
            position: Point3::new(1.0, 0.5, 1.0),
            target: Point3::origin(),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: size.width as f32 / size.height as f32,
            fovy: 1.0,
            near: 0.01,
            far: 200.0,
            is_w_pressed: false,
            is_s_pressed: false,
            is_a_pressed: false,
            is_d_pressed: false,
            yaw: 0.0,
            pitch: 0.0,
            delta: None,
            collision_manager,
        };

        // uniforms
        let mut camera_uniform = CameraUniform::new(camera.position);
        let point_light_uniform = LightUniformArray::new(&lights);
        camera_uniform.update_cam(&camera);

        // buffers
        let point_light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Light Buffer"),
            contents: bytemuck::cast_slice(&[point_light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // textures
        let skybox_texture =
            CubeTexture::from_files(&skybox_files, &device, &queue, Some("Galaxy Texture"));
        let depth_texture = DepthTexture::create_depth_texture(&device, &config, "depth_texture");

        //bind groups
        let camera_bind_group =
            CameraUniform::create_bind_group(&device, &camera_bind_group_layout, &camera_buffer);
        let point_light_bind_group = LightUniformArray::create_bind_group(
            &device,
            &point_light_bind_group_layout,
            &point_light_buffer,
        );
        let skybox_bind_group = CubeTextureBuilder::create_bind_group(
            &device,
            &skybox_texture,
            &skybox_bind_group_layout,
        );

        // pipelines
        let render_pipeline = PipelineFactory::create_render_pipeline(
            &device,
            &render_pipeline_layout,
            config.format,
            Some(DepthTexture::DEPTH_FORMAT),
            &[Vertex::desc(), RawInstance::desc()],
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
            },
        );

        let skybox_render_pipeline = PipelineFactory::create_render_pipeline(
            &device,
            &skybox_pipeline_layout,
            config.format,
            Some(DepthTexture::DEPTH_FORMAT),
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::ShaderModuleDescriptor {
                label: Some("Skybox Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
            },
        );

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
            skybox_bind_group,
            skybox_render_pipeline,
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

            render_pass.set_pipeline(&self.skybox_render_pipeline);
            render_pass.set_bind_group(0, &self.skybox_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update(&mut self) {
        self.camera.update_camera(0.02, 0.004);
        self.camera_uniform.update_cam(&self.camera);
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
