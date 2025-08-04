use nalgebra::{Point3, Vector3};
use pipeline_factory::PipelineFactory;
use shadow_baker::ShadowBaker;
use std::sync::Arc;
use std::time::Duration;
use wgpu::util::DeviceExt;

use wgpu::{
    BindGroup, BindGroupLayout, Buffer, Device, DeviceDescriptor, Queue, RenderPipeline, Surface,
    SurfaceConfiguration,
};
use winit::window::Window;

use crate::camera::Camera;
use crate::camera::camera_uniform::CameraUniform;
use crate::camera::light::Light;
use crate::camera::light_uniform::LightUniformArray;
use crate::camera::shadow_map_uniform::ShadowMapUniform;
use crate::game::collision_manager::CollisionManager;
use crate::game::player::Player;
use crate::game::player_controller::PlayerController;
use crate::model::Model;
use crate::model::cube_texture::{CubeTexture, CubeTextureBuilder};
use crate::model::depth_texture::DepthTexture;
use crate::model::map_loader::MapLoader;
use crate::model::model_instance::RawInstance;
use crate::model::player_model::PlayerModel;
use crate::model::texture::TextureBuilder;
use crate::model::vertex::{LineVertex, Vertex};
use crate::network::Network;

mod pipeline_factory;
mod shadow_baker;

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    models: Vec<Model>,
    lights: Vec<Light>,
    player: Player,
    is_surface_configured: bool,
    debug_lines_len: u32,
    player_controller: PlayerController,
    map_file: String,
    depth_texture: DepthTexture,
    collision_manager: CollisionManager,
    shadow_baker: ShadowBaker,
    player_model_renderer: PlayerModel,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    debug_buffer: Buffer,
    camera_bind_group: BindGroup,
    point_light_bind_group: BindGroup,
    skybox_bind_group: BindGroup,
    shadow_bind_group: BindGroup,
    shadow_bind_group_layout: BindGroupLayout,
    skybox_render_pipeline: RenderPipeline,
    debug_render_pipeline: RenderPipeline,
    shadow_render_pipeline: RenderPipeline,
    render_pipeline: RenderPipeline,
    player_pipeline: RenderPipeline,
}

impl Renderer {
    const MOVE_SPEED: f32 = 2.0;
    const SENSITIVITY: f32 = 0.3;
    const JUMP_STRENGTH: f32 = 1.6;
    const HITBOX_WIDTH: f32 = 0.1;
    pub const FAR_PLANE: f32 = 200.0;
    pub const NEAR_PLANE: f32 = 0.01;
    pub const MAX_PLAYERS: u8 = 32;
    pub const CAMERA_HEIGHT: f32 = 0.5;
    pub async fn new(window: Arc<Window>, map_file: String) -> Result<Self, String> {
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
        let shadow_bind_group_layout = ShadowMapUniform::create_bind_group_layout(&device);
        let shadow_texture_layout = ShadowMapUniform::create_shadow_texture_layout(&device);
        let render_pipeline_layout = PipelineFactory::create_render_pipeline_layout(
            &device,
            &[
                &camera_bind_group_layout,
                &point_light_bind_group_layout,
                &shadow_texture_layout,
                &diffuse_texture_layout,
            ],
        );
        let player_pipeline_layout = PipelineFactory::create_render_pipeline_layout(
            &device,
            &[
                &camera_bind_group_layout, // add lighting for players later
            ],
        );
        let skybox_pipeline_layout = PipelineFactory::create_render_pipeline_layout(
            &device,
            &[&skybox_bind_group_layout, &camera_bind_group_layout],
        );
        let shadow_pipeline_layout =
            PipelineFactory::create_render_pipeline_layout(&device, &[&shadow_bind_group_layout]);
        let debug_pipeline_layout =
            PipelineFactory::create_render_pipeline_layout(&device, &[&camera_bind_group_layout]);

        let map_loader = MapLoader::from_file(&map_file).unwrap();
        let map = map_loader.load(&device, &queue, &diffuse_texture_layout);
        let models = map.models;
        let skybox_files = map.skybox_textures;
        let lights = map.lights;
        let collision_manager = map.collision_manager;
        let debug_lines = map.debug_lines;
        let debug_lines_len = debug_lines.len() as u32;
        let player_head_mesh = map.player_head_mesh;
        let player_body_mesh = map.player_body_mesh;
        let camera = Camera {
            position: Point3::new(1.0, Self::CAMERA_HEIGHT, 1.0),
            target: Point3::new(0.0, Self::CAMERA_HEIGHT, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            aspect: size.width as f32 / size.height as f32,
            fovy: 1.0,
            near: Self::NEAR_PLANE,
            far: Self::FAR_PLANE,
        };
        let player = Player::new(
            Self::SENSITIVITY,
            Self::MOVE_SPEED,
            Self::JUMP_STRENGTH,
            Self::HITBOX_WIDTH,
            Self::CAMERA_HEIGHT,
            camera,
        );
        let player_controller = PlayerController::default();
        let light_ids: Vec<u32> = lights.iter().map(|light| light.id).collect();
        let shadow_baker = ShadowBaker::new(&light_ids, &device);
        let player_model_renderer =
            PlayerModel::new(&device, &[], player_head_mesh, player_body_mesh);

        // uniforms
        let mut camera_uniform = CameraUniform::new(player.camera.position);
        let point_light_uniform = LightUniformArray::new(&lights);
        camera_uniform.update_cam(&player.camera);

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
        let debug_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map1 Debug Buffer"),
            contents: bytemuck::cast_slice(&debug_lines),
            usage: wgpu::BufferUsages::VERTEX,
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
        let shadow_bind_group = ShadowMapUniform::create_shadow_texture_bind_group(
            &device,
            &shadow_baker.shadow_map_texture,
            &shadow_texture_layout,
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
            Some(wgpu::Face::Back),
            true,
            wgpu::CompareFunction::LessEqual,
        );

        let player_pipeline = PipelineFactory::create_render_pipeline(
            &device,
            &player_pipeline_layout,
            config.format,
            Some(DepthTexture::DEPTH_FORMAT),
            &[Vertex::desc(), RawInstance::desc()],
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::ShaderModuleDescriptor {
                label: Some("Player Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/player.wgsl").into()),
            },
            Some(wgpu::Face::Back),
            true,
            wgpu::CompareFunction::LessEqual,
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
            Some(wgpu::Face::Back),
            true,
            wgpu::CompareFunction::LessEqual,
        );

        let debug_render_pipeline = PipelineFactory::create_render_pipeline(
            &device,
            &debug_pipeline_layout,
            config.format,
            Some(DepthTexture::DEPTH_FORMAT),
            &[LineVertex::desc()],
            wgpu::PrimitiveTopology::LineList,
            wgpu::ShaderModuleDescriptor {
                label: Some("Debug Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/debug.wgsl").into()),
            },
            None,
            false,
            wgpu::CompareFunction::Always,
        );

        let shadow_render_pipeline = PipelineFactory::create_shadow_render_pipeline(
            &device,
            &shadow_pipeline_layout,
            Some(CubeTexture::DEPTH_FORMAT),
            &[Vertex::desc(), RawInstance::desc()],
            wgpu::PrimitiveTopology::TriangleList,
            wgpu::ShaderModuleDescriptor {
                label: Some("Shadow Mapping Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shadow.wgsl").into()),
            },
            Some(wgpu::Face::Back),
            true,
            wgpu::CompareFunction::Less,
        );

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: true,
            models,
            lights,
            player,
            collision_manager,
            map_file,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            point_light_bind_group,
            depth_texture,
            render_pipeline,
            skybox_bind_group,
            skybox_render_pipeline,
            player_controller,
            debug_render_pipeline,
            debug_lines_len,
            debug_buffer,
            shadow_render_pipeline,
            shadow_bind_group_layout,
            shadow_bind_group,
            shadow_baker,
            player_pipeline,
            player_model_renderer,
        })
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        // Shadow render pass
        for light in &self.lights {
            self.shadow_baker.update_light_shadow_map(
                light,
                &self.device,
                &self.queue,
                &self.models,
                &self.shadow_render_pipeline,
                &self.shadow_bind_group_layout,
            );
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
            render_pass.set_bind_group(2, &self.shadow_bind_group, &[]);
            for model in &self.models {
                model.draw(&mut render_pass);
            }
            render_pass.set_pipeline(&self.player_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            self.player_model_renderer.draw(&mut render_pass);

            render_pass.set_pipeline(&self.skybox_render_pipeline);
            render_pass.set_bind_group(0, &self.skybox_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.draw(0..3, 0..1);

            if self.player_controller.debug_enabled {
                render_pass.set_pipeline(&self.debug_render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.debug_buffer.slice(..));
                render_pass.draw(0..self.debug_lines_len, 0..1);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update(&mut self, dt: Duration, network_handler: &Option<Network>) {
        if let Some(network_handler) = network_handler {
            let player_states = network_handler
                .player_states
                .values()
                .cloned()
                .collect::<Vec<_>>();
            self.player_model_renderer
                .update(&self.queue, &player_states);
        }
        self.player
            .update(dt, &mut self.collision_manager, &mut self.player_controller);
        self.camera_uniform.update_cam(&self.player.camera);
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

    pub fn rerender(&mut self) {
        let diffuse_texture_layout = TextureBuilder::create_bind_group_layout(&self.device);
        let skybox_bind_group_layout = CubeTextureBuilder::create_bind_group_layout(&self.device);
        let point_light_bind_group_layout =
            LightUniformArray::create_bind_group_layout(&self.device);

        let map_loader = MapLoader::from_file(&self.map_file).unwrap();
        let map = map_loader.load(&self.device, &self.queue, &diffuse_texture_layout);
        let models = map.models;
        let skybox_files = map.skybox_textures;
        let lights = map.lights;
        let collision_manager = map.collision_manager;
        let debug_lines = map.debug_lines;
        let debug_lines_len = debug_lines.len() as u32;

        let point_light_uniform = LightUniformArray::new(&lights);

        let point_light_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Point Light Buffer"),
                    contents: bytemuck::cast_slice(&[point_light_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
        let debug_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Map1 Debug Buffer"),
                contents: bytemuck::cast_slice(&debug_lines),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let point_light_bind_group = LightUniformArray::create_bind_group(
            &self.device,
            &point_light_bind_group_layout,
            &point_light_buffer,
        );
        let skybox_texture = CubeTexture::from_files(
            &skybox_files,
            &self.device,
            &self.queue,
            Some("Skybox Texture"),
        );
        let skybox_bind_group = CubeTextureBuilder::create_bind_group(
            &self.device,
            &skybox_texture,
            &skybox_bind_group_layout,
        );
        self.skybox_bind_group = skybox_bind_group;
        self.models = models;
        self.point_light_bind_group = point_light_bind_group;
        self.debug_buffer = debug_buffer;
        self.debug_lines_len = debug_lines_len;
        self.collision_manager = collision_manager;
        self.shadow_baker.update_scene_version();
        for light in &self.lights {
            self.shadow_baker.update_light_version_from_id(light.id);
        }
    }

    pub fn get_mut_player_controller(&mut self) -> &mut PlayerController {
        &mut self.player_controller
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }
}
