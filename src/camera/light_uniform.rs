use nalgebra::Point3;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PointLightUniform {
    pub position: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 4],
}

impl PointLightUniform {
    pub fn new(position: Point3<f32>, intensity: f32) -> Self {
        Self {
            position: position.into(),
            intensity,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn _update_position(&mut self, new_position: Point3<f32>) {
        self.position = new_position.into();
    }

    pub fn update_intensity(&mut self, new_intensity: f32) {
        self.intensity = new_intensity;
    }

    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("point_light_bind_group_layout"),
        })
    }

    pub fn create_bind_group(
        device: &Device,
        point_light_bind_group_layout: &BindGroupLayout,
        light_buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: point_light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: Some("point_light_bind_group"),
        })
    }
}
