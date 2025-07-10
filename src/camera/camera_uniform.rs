use nalgebra::{Matrix4, Point3, Point4};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};

use super::Camera;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_pos: [f32; 4],
    view: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    inv_proj: [[f32; 4]; 4],
    inv_view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new(cam_pos: Point3<f32>) -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
            view_pos: cam_pos.to_homogeneous().into(),
            view: Matrix4::identity().into(),
            inv_proj: Matrix4::identity().into(),
            inv_view: Matrix4::identity().into(),
        }
    }

    pub fn update_cam(&mut self, camera: &Camera) {
        self.view_pos = camera.position.to_homogeneous().into();
        let view = camera.get_view_mat();
        let proj = camera.get_proj_mat();
        self.view_proj = (proj * view).into();
        self.view = view.into();
        self.inv_proj = proj.try_inverse().unwrap().into();
        self.inv_view = view.try_inverse().unwrap().into();
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
            label: Some("camera_bind_group_layout"),
        })
    }

    pub fn create_bind_group(
        device: &Device,
        camera_bind_group_layout: &BindGroupLayout,
        camera_buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        })
    }
}
