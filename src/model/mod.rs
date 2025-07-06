use texture::Texture;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Buffer, Device, Queue};
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
}

impl Model {
    pub fn sample(device: &Device, queue: &Queue, bind_group_layout: &BindGroupLayout) -> Self {
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

        let num_vertices = vertices.len() as u32;

        let indices: Vec<u16> = vec![
            // Front
            0, 1, 2, 0, 2, 3, // Back
            4, 5, 6, 4, 6, 7, // Left
            8, 9, 10, 8, 10, 11, // Right
            12, 13, 14, 12, 14, 15, // Top
            16, 17, 18, 16, 18, 19, // Bottom
            20, 21, 22, 20, 22, 23,
        ];

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

        let texture = Texture::from_file(
            "textures/cube-diffuse.jpg",
            device,
            queue,
            Some("sample_texture"),
        );
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
            num_elements: num_vertices,
            material: 0,
        };
        Self {
            meshes: vec![mesh],
            materials: vec![material],
        }
    }
}
