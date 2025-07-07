use nalgebra::{Matrix4, Vector3};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Buffer, Device, Queue};

use crate::model::texture::Texture;
use crate::model::{Material, Mesh};
use crate::model::{Model, vertex::Vertex};
use crate::renderer::model_instance::{Instance, RawInstance};

pub struct Map1;

impl Map1 {
    pub const WIDTH: u32 = 8;
    pub const HEIGHT: u32 = 8;
    pub fn get_models(
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
    ) -> Vec<Model> {
        let floor_material =
            Self::load_texture("textures/map1/floor.png", device, queue, bind_group_layout);
        let wall_material =
            Self::load_texture("textures/map1/wall.png", device, queue, bind_group_layout);
        let ceil_material =
            Self::load_texture("textures/map1/ceil.png", device, queue, bind_group_layout);

        let ccw: [u16; 6] = [0, 2, 1, 0, 3, 2];
        let cw: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let floor = vec![
            Vertex {
                position: [-0.5, 0.0, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.0, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.0, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
        ];

        let wall = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
        ];

        let (floor_instance_buffer, floor_num_instances) = Self::floor_instance(device);
        vec![Model {
            meshes: vec![Self::gen_mesh(&floor, &ccw, 0, device)],
            materials: vec![floor_material],
            instance_buffer: floor_instance_buffer,
            num_instances: floor_num_instances,
        }]
    }

    fn floor_instance(device: &Device) -> (Buffer, u32) {
        let instances: Vec<Instance> = (0..(Self::WIDTH * Self::HEIGHT))
            .map(|pos| -> Instance {
                Instance {
                    model_mat: Matrix4::new_translation(&Vector3::new(
                        (pos % Self::WIDTH) as f32,
                        0.0,
                        (pos / Self::HEIGHT) as f32,
                    )),
                }
            })
            .collect();
        let num_instances = instances.len() as u32;
        let raw_instances: Vec<RawInstance> =
            instances.iter().map(|instance| instance.to_raw()).collect();
        (
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&raw_instances),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            num_instances,
        )
    }

    fn gen_mesh(vertices: &[Vertex], indices: &[u16], material: u32, device: &Device) -> Mesh {
        let num_indices = indices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map1 Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Map1 Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Mesh {
            name: String::from("sample_mesh"),
            vertex_buffer,
            index_buffer,
            num_elements: num_indices,
            material,
        }
    }

    fn load_texture(
        filename: &str,
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
    ) -> Material {
        let texture = Texture::from_file(filename, device, queue, Some(filename));
        let bind_group = texture.create_bind_group(device, bind_group_layout);

        Material {
            name: String::from(filename),
            diffuse_texture: texture,
            bind_group,
        }
    }
}
