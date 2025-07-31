use nalgebra::{Matrix3, Matrix4, Point3, Vector3};
use wgpu::{Buffer, Device, Queue, RenderPass};

use crate::{network::player_state::TimedPlayerState, renderer::Renderer};

use super::{Mesh, model_instance::RawInstance, vertex::Vertex};
use wgpu::util::DeviceExt;

pub struct PlayerModel {
    pub meshes: Vec<Mesh>,
    pub instances: Vec<RawInstance>,
    pub instance_buffer: Buffer,
    pub num_instances: u32,
}

impl PlayerModel {
    pub fn new(device: &Device, player_states: &[TimedPlayerState]) -> Self {
        // TEMP hardcoded player vertices to just test. this should form a cube.
        let vertices = vec![
            // Front face
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 1.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            // Back face
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, 0.0, -1.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            // Top face
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [0.0, 1.0, 0.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            // Bottom face
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [0.0, -1.0, 0.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            // Right face
            Vertex {
                position: [0.5, -0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                normal: [1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            // Left face
            Vertex {
                position: [-0.5, -0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                normal: [-1.0, 0.0, 0.0],
                tex_coords: [0.0, 1.0],
                tangent: [0.0, 0.0, 0.0],
                bitangent: [0.0, 0.0, 0.0],
            },
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 2, 3, 0, // Front
            4, 5, 6, 6, 7, 4, // Back
            8, 9, 10, 10, 11, 8, // Top
            12, 13, 14, 14, 15, 12, // Bottom
            16, 17, 18, 18, 19, 16, // Right
            20, 21, 22, 22, 23, 20, // Left
        ];

        let mut instances: Vec<RawInstance> =
            player_states.iter().map(Self::compute_instance).collect();
        let num_instances = instances.len() as u32;
        instances.resize(
            Renderer::MAX_PLAYERS as usize,
            RawInstance {
                model_mat: Matrix4::identity().into(),
                normal_mat: Matrix3::identity().into(),
            },
        );

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Player Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let meshes = vec![Mesh {
            name: String::from("Player Mesh"),
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Player Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Player Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            num_elements: indices.len() as u32,
            material: None,
        }];

        Self {
            meshes,
            instances,
            instance_buffer,
            num_instances,
        }
    }

    pub fn update(&mut self, queue: &Queue, player_states: &[TimedPlayerState]) {
        self.instances = player_states.iter().map(Self::compute_instance).collect();
        self.num_instances = self.instances.len() as u32;
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }

    pub fn draw(&self, render_pass: &mut RenderPass) {
        if self.num_instances == 0 {
            return;
        }
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        for mesh in &self.meshes {
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_elements, 0, 0..self.num_instances);
        }
    }

    fn compute_instance(timed_player_state: &TimedPlayerState) -> RawInstance {
        let player_state = timed_player_state.player_state;
        let dt = timed_player_state.time.elapsed().as_secs_f32();
        let new_pos =
            Point3::from(player_state.position) + Vector3::from(player_state.velocity) * dt;
        RawInstance {
            model_mat: Matrix4::new_translation(&new_pos.coords).into(),
            normal_mat: Matrix3::identity().into(),
        }
    }
}
