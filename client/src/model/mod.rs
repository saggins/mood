#![allow(dead_code)]
use std::{collections::HashMap, sync::Arc};

use model_instance::RawInstance;
use wgpu::{Buffer, RenderPass};

pub mod cube_texture;
pub mod depth_texture;
pub mod map_loader;
pub mod model_instance;
pub mod player_model;
pub mod texture;
pub mod vertex;

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_elements: u32,
    pub material: Option<String>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Arc<HashMap<String, Material>>,
    pub instances: Vec<RawInstance>,
    pub instance_buffer: Buffer,
    pub num_instances: u32,
}

impl Model {
    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        for mesh in &self.meshes {
            if let Some(ref material) = mesh.material {
                render_pass.set_bind_group(
                    3,
                    &self.materials.get(material).unwrap().bind_group,
                    &[],
                );
            } else {
                todo!(); // create default material for objects without texture.
            }
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.num_instances);
        }
    }

    pub fn draw_shadow(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        for mesh in &self.meshes {
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.num_instances);
        }
    }
}
