use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
use wgpu::{BindGroup, BindGroupLayout, Device};

use crate::model::cube_texture::CubeTexture;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ShadowMapUniform {
    pub view_proj: [[f32; 4]; 4],
    pub position: [f32; 3],
    _padding: f32,
}

impl ShadowMapUniform {
    pub fn get_uniform_map_for_face(light_pos: Point3<f32>, face_index: u32) -> Self {
        let eye = light_pos;
        let (target, up): (Point3<f32>, Vector3<f32>) = match face_index {
            0 => (eye + Vector3::x(), -Vector3::y()), // +X
            1 => (eye - Vector3::x(), -Vector3::y()), // -X
            2 => (eye + Vector3::y(), Vector3::z()),  // +Y
            3 => (eye - Vector3::y(), -Vector3::z()), // -Y
            4 => (eye + Vector3::z(), -Vector3::y()), // +Z
            5 => (eye - Vector3::z(), -Vector3::y()), // -Z
            _ => panic!("Invalid cube face index"),
        };

        let proj = Perspective3::new(1.0, std::f32::consts::FRAC_PI_2, 0.1, 200.0);

        let view = Matrix4::look_at_rh(&eye, &target, &up);

        Self {
            view_proj: (proj.to_homogeneous() * view).into(),
            position: light_pos.into(),
            _padding: 0.0,
        }
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("shadow_map_bind_group_layout"),
        })
    }

    pub fn create_shadow_texture_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::CubeArray,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
            label: Some("shadow_bind_group_layout"),
        })
    }
    pub fn create_shadow_texture_bind_group(
        device: &Device,
        shadow_texture: &CubeTexture,
        shadow_texture_bind_group_layout: &BindGroupLayout,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: shadow_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&shadow_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&shadow_texture.sampler),
                },
            ],
            label: Some("shadow_bind_group"),
        })
    }
}
