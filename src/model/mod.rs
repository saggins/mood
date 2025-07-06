use crate::renderer::model_instance::{Instance, RawInstance};
use nalgebra::{Matrix, Matrix4, Vector3};
use texture::Texture;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Buffer, Device, Queue, RenderPass};

pub mod depth_texture;
pub mod texture;
pub mod vertex;

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub num_elements: u32,
    pub material: u32,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub instance_buffer: Buffer,
    pub num_instances: u32,
}

impl Model {
    pub fn floors(device: &Device, queue: &Queue, bind_group_layout: &BindGroupLayout) -> Self {
        let vertices = vec![
            Vertex {
                position: [-1.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let indices: Vec<u16> = vec![0, 2, 1, 0, 3, 2];
        let num_indices = indices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let texture =
            Texture::from_file("textures/floor.png", device, queue, Some("sample_texture2"));
        let bind_group = texture.create_bind_group(device, bind_group_layout);

        let material = Material {
            name: String::from("floor"),
            diffuse_texture: texture,
            bind_group,
        };

        let mesh = Mesh {
            name: String::from("Floor"),
            vertex_buffer,
            index_buffer,
            num_elements: num_indices,
            material: 0,
        };
        let instances: Vec<Instance> = (0..255)
            .map(|pos| -> Instance {
                Instance {
                    model_mat: Matrix4::new_translation(&Vector3::new(
                        (pos % 16) as f32,
                        -0.5,
                        (pos / 16) as f32,
                    )),
                }
            })
            .collect();
        let num_instances = instances.len() as u32;
        let raw_instances: Vec<RawInstance> =
            instances.iter().map(|instance| instance.to_raw()).collect();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self {
            meshes: vec![mesh],
            materials: vec![material],
            instance_buffer,
            num_instances,
        }
    }

    pub fn walls(device: &Device, queue: &Queue, bind_group_layout: &BindGroupLayout) -> Self {
        let vertices = vec![
            // Front face (Z+)
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (Z-)
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Left face (X-)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            // Right face (X+)
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            // Top face (Y+)
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            // Bottom face (Y-)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];

        let indices: Vec<u16> = vec![
            // Front
            0, 1, 2, 0, 2, 3, // Back
            4, 5, 6, 4, 6, 7, // Left
            8, 9, 10, 8, 10, 11, // Right
            12, 13, 14, 12, 14, 15, // Top
            16, 17, 18, 16, 18, 19, // Bottom
            20, 21, 22, 20, 22, 23,
        ];
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let texture =
            Texture::from_file("textures/wall.png", device, queue, Some("sample_texture"));
        let bind_group = texture.create_bind_group(device, bind_group_layout);

        let material = Material {
            name: String::from("rocks"),
            diffuse_texture: texture,
            bind_group,
        };

        let mesh = Mesh {
            name: String::from("Wall"),
            vertex_buffer,
            index_buffer,
            num_elements: num_indices,
            material: 0,
        };
        let instances = [
            Instance {
                model_mat: Matrix4::identity(),
            },
            Instance {
                model_mat: Matrix4::new_translation(&Vector3::new(1.0, 0.0, 0.0)),
            },
            Instance {
                model_mat: Matrix4::new_translation(&Vector3::new(2.0, 0.0, 0.0)),
            },
            Instance {
                model_mat: Matrix4::new_translation(&Vector3::new(3.0, 0.0, 0.0)),
            },
            Instance {
                model_mat: Matrix4::new_translation(&Vector3::new(4.0, 0.0, 0.0)),
            },
            Instance {
                model_mat: Matrix4::new_translation(&Vector3::new(5.0, 0.0, 0.0)),
            },
        ];
        let num_instances = instances.len() as u32;
        let raw_instances: Vec<RawInstance> =
            instances.iter().map(|instance| instance.to_raw()).collect();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&raw_instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        Self {
            meshes: vec![mesh],
            materials: vec![material],
            instance_buffer,
            num_instances,
        }
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        for mesh in &self.meshes {
            render_pass.set_bind_group(2, &self.materials[mesh.material as usize].bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.num_instances);
        }
    }
}
