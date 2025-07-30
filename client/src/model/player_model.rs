use std::time::Duration;

use bincode::config::IntEncoding;
use nalgebra::{Matrix3, Matrix4, Point3, Vector3};
use wgpu::{Buffer, Device, Queue};

use crate::network::player_state::TimedPlayerState;

use super::{Mesh, model_instance::RawInstance};
use wgpu::util::DeviceExt;

pub struct PlayerModel {
    pub mesh: Mesh,
    pub instances: Vec<RawInstance>,
    pub instance_buffer: Buffer,
    pub num_instances: u32,
}

impl PlayerModel {
    pub fn new(device: &Device, player_states: &[TimedPlayerState]) -> Self {
        let instances: Vec<RawInstance> =
            player_states.iter().map(Self::compute_instance).collect();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let mesh = Mesh {
            name: String::from("Player Mesh"),
            vertex_buffer: todo!(), // create mesh for players
            index_buffer: todo!(),
            num_elements: todo!(),
            material: None,
        };
        Self {
            mesh,
            instances,
            instance_buffer,
            num_instances: instances.len() as u32,
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

    pub fn draw(&self) {
        todo!();
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
