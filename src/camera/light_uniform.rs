use log::info;
use nalgebra::Point3;
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device};

use super::light::Light;

const MAX_LIGHTS: usize = 32;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    _padding: f32,
    pub color: [f32; 3],
    pub intensity: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniformArray {
    pub lights: [LightUniform; MAX_LIGHTS],
    pub count: u32,
    pub _padding: [f32; 3],
}

impl LightUniformArray {
    pub fn new(lights: &[Light]) -> Self {
        if lights.len() > MAX_LIGHTS {
            info!("More than {MAX_LIGHTS} lights");
            panic!();
        }
        let mut light_array = [LightUniform::new(Point3::origin(), 0.0); MAX_LIGHTS];
        for i in 0..lights.len() {
            light_array[i].position = lights[i].position.into();
            light_array[i].intensity = lights[i].intensity;
            light_array[i].color = lights[i].color;
        }
        Self {
            count: lights.len() as u32,
            _padding: [0.0, 0.0, 0.0],
            lights: light_array,
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

impl LightUniform {
    pub fn new(position: Point3<f32>, intensity: f32) -> Self {
        Self {
            position: position.into(),
            _padding: 0.0,
            color: [0.0, 0.0, 0.0],
            intensity,
        }
    }
}
