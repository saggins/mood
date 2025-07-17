use nalgebra::{Matrix3, Point3, Vector3};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, error::Error, fs};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    camera::light::Light,
    game::{bounding_box::BoundingBox, collision_manager::CollisionManager},
};

use super::model_instance::{Instance, RawInstance};
use super::{
    Material, Mesh, Model,
    texture::TextureBuilder,
    vertex::{LineVertex, Vertex},
};

pub struct Map {
    pub models: Vec<Model>,
    pub skybox_textures: Vec<String>,
    pub lights: Vec<Light>,
    pub collision_manager: CollisionManager,
    pub debug_lines: Vec<LineVertex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapLoader {
    skybox: Vec<String>,
    lights: Vec<LightLoader>,
    materials: Vec<MaterialLoader>,
    models: Vec<ModelLoader>,
    bounding_boxes: Vec<BoundingBoxLoader>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MaterialLoader {
    pub name: String,
    pub texture_map: String,
    pub normal_map: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LightLoader {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelLoader {
    pub meshes: Vec<MeshLoader>,
    pub instances: Vec<InstanceLoader>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MeshLoader {
    pub name: String,
    pub vertices: Vec<VertexLoader>,
    pub indices: Vec<u16>,
    pub material: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VertexLoader {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug)]
struct InstanceLoader {
    pub is_grid: bool,
    pub position: [f32; 3],
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub rotation: [[f32; 3]; 3],
}

#[derive(Serialize, Deserialize, Debug)]
struct BoundingBoxLoader {
    pub top_left: [f32; 3],
    pub bottom_right: [f32; 3],
    pub collide_on_top: bool,
}

impl MapLoader {
    const LINE_COLOR: [f32; 3] = [1.0, 0.0, 0.0];
    const MATERIAL_INDEX: u32 = 0;
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let json_data = fs::read_to_string(filename)?;
        let l: Self = serde_json::from_str(&json_data)?;
        Ok(l)
    }

    pub fn load(&self, device: &Device, queue: &Queue, bind_group_layout: &BindGroupLayout) -> Map {
        let skybox_textures = self.skybox.clone();
        let lights: Vec<Light> = self
            .lights
            .iter()
            .map(|light| -> Light {
                Light {
                    position: Point3::new(light.position[0], light.position[1], light.position[2]),
                    color: light.color,
                    intensity: light.intensity,
                }
            })
            .collect();
        let map_boxes: Vec<BoundingBox> = self
            .bounding_boxes
            .iter()
            .map(|bounding_box| -> BoundingBox {
                BoundingBox {
                    top_left: Point3::new(
                        bounding_box.top_left[0],
                        bounding_box.top_left[1],
                        bounding_box.top_left[2],
                    ),
                    bottom_right: Point3::new(
                        bounding_box.bottom_right[0],
                        bounding_box.bottom_right[1],
                        bounding_box.bottom_right[2],
                    ),
                    collide_on_top: bounding_box.collide_on_top,
                }
            })
            .collect();

        let debug_lines: Vec<LineVertex> = map_boxes
            .iter()
            .flat_map(|map_box| Self::bounding_box_to_line_vertices(map_box, Self::LINE_COLOR))
            .collect();
        let collision_manager = CollisionManager { map_boxes };

        let materials: Arc<HashMap<String, Material>> = Arc::new(
            self.materials
                .par_iter()
                .map(|material| -> (String, Material) {
                    (
                        String::from(&material.name),
                        Self::load_texture(
                            &material.texture_map,
                            &material.normal_map,
                            device,
                            queue,
                            bind_group_layout,
                        ),
                    )
                })
                .collect(),
        );

        let models: Vec<Model> = self
            .models
            .iter()
            .map(|model| -> Model {
                let meshes: Vec<Mesh> = model
                    .meshes
                    .iter()
                    .map(|mesh| -> Mesh {
                        let mut vertices: Vec<Vertex> = mesh
                            .vertices
                            .iter()
                            .map(|vertex| -> Vertex {
                                Vertex {
                                    position: vertex.position,
                                    tex_coords: vertex.tex_coords,
                                    normal: vertex.normal,
                                    tangent: [0.0; 3],
                                    bitangent: [0.0; 3],
                                }
                            })
                            .collect();
                        Self::gen_mesh(
                            &mesh.name,
                            &mut vertices,
                            &mesh.indices,
                            &mesh.material,
                            device,
                        )
                    })
                    .collect();

                let instances: Vec<RawInstance> = model
                    .instances
                    .iter()
                    .flat_map(|instance| -> Vec<RawInstance> {
                        let mut instances = vec![];
                        for index in 0..(instance.depth * instance.height * instance.width) {
                            let i = index / (instance.height * instance.width);
                            let j = (index % (instance.height * instance.width)) / instance.width;
                            let k = index % instance.width;

                            instances.push(
                                Instance {
                                    position: Vector3::new(
                                        instance.position[0] + k as f32,
                                        instance.position[1] + j as f32,
                                        instance.position[2] + i as f32,
                                    ),
                                    rotation: Matrix3::from(instance.rotation),
                                }
                                .to_raw(),
                            )
                        }
                        instances
                    })
                    .collect();
                let num_instances = instances.len() as u32;
                let instance_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Index Buffer"),
                        contents: bytemuck::cast_slice(&instances),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
                Model {
                    meshes,
                    materials: materials.clone(),
                    instance_buffer,
                    num_instances,
                }
            })
            .collect();

        Map {
            skybox_textures,
            collision_manager,
            lights,
            debug_lines,
            models,
        }
    }
    fn bounding_box_to_line_vertices(bbox: &BoundingBox, color: [f32; 3]) -> Vec<LineVertex> {
        let top_left = bbox.top_left;
        let bottom_right = bbox.bottom_right;

        let corners = [
            [top_left.x, bottom_right.y, top_left.z],
            [bottom_right.x, bottom_right.y, top_left.z],
            [bottom_right.x, bottom_right.y, bottom_right.z],
            [top_left.x, bottom_right.y, bottom_right.z],
            [top_left.x, top_left.y, top_left.z],
            [bottom_right.x, top_left.y, top_left.z],
            [bottom_right.x, top_left.y, bottom_right.z],
            [top_left.x, top_left.y, bottom_right.z],
        ];

        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        let mut vertices = vec![];
        for (start_idx, end_idx) in edges {
            vertices.push(LineVertex {
                position: corners[start_idx],
                color,
            });
            vertices.push(LineVertex {
                position: corners[end_idx],
                color,
            });
        }

        vertices
    }

    fn load_texture(
        filename: &str,
        normal_filename: &str,
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
    ) -> Material {
        let diffuse_texture =
            super::texture::Texture::from_file(filename, device, queue, Some(filename));
        let normal_texture = super::texture::Texture::from_file(
            normal_filename,
            device,
            queue,
            Some(normal_filename),
        );
        let bind_group = TextureBuilder::create_bind_group(
            device,
            &diffuse_texture,
            &normal_texture,
            bind_group_layout,
        );

        Material {
            name: String::from(filename),
            diffuse_texture,
            normal_texture,
            bind_group,
        }
    }
    fn gen_mesh(
        name: &str,
        vertices: &mut [Vertex],
        indices: &[u16],
        material: &str,
        device: &Device,
    ) -> Mesh {
        let mut triangles_included = vec![0; vertices.len()];
        for tri in indices.chunks(3) {
            let t1 = tri[0] as usize;
            let t2 = tri[1] as usize;
            let t3 = tri[2] as usize;
            let v1 = vertices[t1];
            let v2 = vertices[t2];
            let v3 = vertices[t3];

            let (edge1, uv1) = v2 - v1;
            let (edge2, uv2) = v3 - v1;
            let r = 1.0 / (uv1.x * uv2.y - uv1.y * uv2.x);
            let tangent = (edge1 * uv2.y - edge2 * uv1.y) * r;
            let bitangent = (edge2 * uv1.x - edge1 * uv2.x) * r;
            vertices[t1].tangent = (tangent + Vector3::from(vertices[t1].tangent)).into();
            vertices[t2].tangent = (tangent + Vector3::from(vertices[t2].tangent)).into();
            vertices[t3].tangent = (tangent + Vector3::from(vertices[t3].tangent)).into();
            vertices[t1].bitangent = (bitangent + Vector3::from(vertices[t1].bitangent)).into();
            vertices[t2].bitangent = (bitangent + Vector3::from(vertices[t2].bitangent)).into();
            vertices[t3].bitangent = (bitangent + Vector3::from(vertices[t3].bitangent)).into();
            triangles_included[t1] += 1;
            triangles_included[t2] += 1;
            triangles_included[t3] += 1;
        }
        for (i, n) in triangles_included.into_iter().enumerate() {
            let denom = 1.0 / n as f32;
            let v = &mut vertices[i];
            v.tangent = (Vector3::from(v.tangent) * denom).into();
            v.bitangent = (Vector3::from(v.bitangent) * denom).into();
        }

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
            name: String::from(name),
            vertex_buffer,
            index_buffer,
            num_elements: num_indices,
            material: String::from(material),
        }
    }
}
