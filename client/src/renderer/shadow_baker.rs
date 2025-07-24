use rand::random;
use std::collections::HashMap;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, Queue, RenderPipeline};

use crate::camera::shadow_map_uniform::ShadowMapUniform;
use crate::{
    camera::light::Light,
    model::{Model, cube_texture::CubeTexture},
};

pub struct ShadowBaker {
    pub shadow_map_texture: CubeTexture,
    cached_shadow_maps: HashMap<u32, CachedShadowMap>,
    scene_version: u64,
    light_versions: HashMap<u32, u64>,
}

pub struct CachedShadowMap {
    pub scene_version: u64,
    pub light_version: u64,
    // Togged on the first render since we always need to update the scene the first time.
    pub init: bool,
}

impl ShadowBaker {
    const RESOLUTION: u32 = 1024;
    const INIT_VERSION: u64 = 0;
    pub fn new(light_ids: &[u32], device: &Device) -> Self {
        let light_versions = light_ids
            .iter()
            .map(|id| (*id, Self::INIT_VERSION))
            .collect();
        let num_lights = light_ids.len();
        let shadow_map_texture = CubeTexture::new_shadow_map(
            device,
            Self::RESOLUTION,
            num_lights as u32,
            Some("Shadow Map"),
        );
        let cached_shadow_maps = light_ids
            .iter()
            .map(|id| {
                (
                    *id,
                    CachedShadowMap {
                        scene_version: Self::INIT_VERSION,
                        light_version: Self::INIT_VERSION,
                        init: false,
                    },
                )
            })
            .collect();
        Self {
            cached_shadow_maps,
            shadow_map_texture,
            scene_version: Self::INIT_VERSION,
            light_versions,
        }
    }

    pub fn update_light_shadow_map(
        &mut self,
        light: &Light,
        device: &Device,
        queue: &Queue,
        models: &[Model],
        shadow_pipeline: &RenderPipeline,
        shadow_bind_group_layout: &BindGroupLayout,
    ) {
        let current_scene_version = self.scene_version;
        let current_light_version = *self.light_versions.get(&light.id).unwrap();

        let needs_rebake = self
            .cached_shadow_maps
            .get(&light.id)
            .iter()
            .all(|cached_shadow| {
                cached_shadow.scene_version != current_scene_version
                    || cached_shadow.light_version != current_light_version
                    || !cached_shadow.init
            });

        if needs_rebake {
            self.bake_shadows(
                device,
                queue,
                models,
                light,
                shadow_pipeline,
                shadow_bind_group_layout,
            );
            let cached_shadow_map = self.cached_shadow_maps.get_mut(&light.id).unwrap();
            cached_shadow_map.scene_version = current_scene_version;
            cached_shadow_map.light_version = current_light_version;
        }
    }

    fn bake_shadows(
        &self,
        device: &Device,
        queue: &Queue,
        models: &[Model],
        light: &Light,
        shadow_pipeline: &RenderPipeline,
        shadow_bind_group_layout: &BindGroupLayout,
    ) {
        for face_index in 0..6 {
            let shadow_map_uniform =
                ShadowMapUniform::get_uniform_map_for_face(light.position, face_index);
            let light_camera_uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Shadow ViewProj Buffer"),
                    contents: bytemuck::cast_slice(&[shadow_map_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

            let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: shadow_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_camera_uniform_buffer.as_entire_binding(),
                }],
                label: Some("Shadow Bind Group"),
            });

            let face_view = self.shadow_map_texture.create_view_from_face(
                light.id,
                face_index,
                Some("shadow map face view"),
            );

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Shadow Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Shadow Render Pass"),
                    color_attachments: &[],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &face_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    ..Default::default()
                });

                render_pass.set_pipeline(shadow_pipeline);
                render_pass.set_bind_group(0, &light_bind_group, &[]);
                for model in models {
                    model.draw_shadow(&mut render_pass);
                }
            }

            queue.submit(Some(encoder.finish()));
        }
    }

    pub fn update_scene_version(&mut self) {
        self.scene_version += 1;
    }

    pub fn update_light_version_from_id(&mut self, light_id: u32) {
        self.light_versions.insert(light_id, random::<u64>());
    }
}
