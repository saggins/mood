use nalgebra::{Matrix3, Point3, Rotation3, Vector3};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Buffer, Device, Queue};

use crate::camera::light::Light;
use crate::game::bounding_box::BoundingBox;
use crate::game::collision_manager::CollisionManager;
use crate::model::model_instance::{Instance, RawInstance};
use crate::model::texture::{Texture, TextureBuilder};
use crate::model::vertex::LineVertex;
use crate::model::{Material, Mesh};
use crate::model::{Model, vertex::Vertex};

/// NOTE THIS FILE IS FOR TESTING ONLY
/// IT CONTAINS HARD CODED MESH VALUES
/// WHICH *SHOULD* BE LOADED FROM A FILE
/// LATER. THIS IS JUST FOR TESTING
pub struct Map1;

impl Map1 {
    pub const WIDTH: u32 = 8;
    pub const HEIGHT: u32 = 8;
    pub fn get_models(
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
    ) -> (
        Vec<Model>,
        Vec<String>,
        Vec<Light>,
        CollisionManager,
        Vec<LineVertex>,
    ) {
        let floor_material = Self::load_texture(
            "textures/map1/sand.png",
            "textures/map1/sand_normal.png",
            device,
            queue,
            bind_group_layout,
        );
        let wall_material = Self::load_texture(
            "textures/map1/bricks.png",
            "textures/map1/bricks_normal.png",
            device,
            queue,
            bind_group_layout,
        );
        let ccw: [u16; 6] = [0, 2, 1, 0, 3, 2];
        let _cw: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let mut floor = vec![
            Vertex {
                position: [-0.5, 0.0, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [0.5, 0.0, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [0.5, 0.0, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [-0.5, 0.0, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
        ];

        let mut wall = vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
                tangent: [0.0; 3],
                bitangent: [0.0; 3],
            },
        ];

        let map_boxes = vec![Self::floor_box(), Self::wall_box(), Self::wall_box_2()];
        let mut debug_map_box_lines = vec![];
        for map_box in &map_boxes {
            let box_debug_lines = Self::bounding_box_to_line_vertices(map_box, [1.0, 0.0, 0.0]);
            for box_debug_line in box_debug_lines {
                debug_map_box_lines.push(box_debug_line);
            }
        }

        let collsion_manager = CollisionManager { map_boxes };

        let (floor_instance_buffer, floor_num_instances) = Self::floor_instance(device);
        let (outer_wall_instance_buffer, outer_wall_num_instances) =
            Self::outer_wall_instance(device);
        (
            vec![
                Model {
                    meshes: vec![Self::gen_mesh(&mut floor, &ccw, 0, device)],
                    materials: vec![floor_material],
                    instance_buffer: floor_instance_buffer,
                    num_instances: floor_num_instances,
                },
                Model {
                    meshes: vec![Self::gen_mesh(&mut wall, &ccw, 0, device)],
                    materials: vec![wall_material],
                    instance_buffer: outer_wall_instance_buffer,
                    num_instances: outer_wall_num_instances,
                },
            ],
            vec![
                String::from("textures/map1/skybox/right.jpg"),
                String::from("textures/map1/skybox/left.jpg"),
                String::from("textures/map1/skybox/top.jpg"),
                String::from("textures/map1/skybox/bottom.jpg"),
                String::from("textures/map1/skybox/front.jpg"),
                String::from("textures/map1/skybox/back.jpg"),
            ],
            vec![
                Light {
                    position: Point3::new(3.0, 0.5, 0.5),
                    intensity: 1.0,
                    color: [1.0, 0.0, 0.0],
                },
                Light {
                    position: Point3::new(3.0, 0.5, 3.0),
                    intensity: 1.0,
                    color: [0.0, 1.0, 0.0],
                },
                Light {
                    position: Point3::new(6.0, 0.5, 2.0),
                    intensity: 1.0,
                    color: [0.0, 0.0, 1.0],
                },
            ],
            collsion_manager,
            debug_map_box_lines,
        )
    }

    fn bounding_box_to_line_vertices(bbox: &BoundingBox, color: [f32; 3]) -> Vec<LineVertex> {
        let top_left = bbox.top_left;
        let bottom_right = bbox.bottom_right;

        // Calculate the 8 corners of the bounding box
        let corners = [
            [top_left.x, bottom_right.y, top_left.z],
            [bottom_right.x, bottom_right.y, top_left.z],
            [bottom_right.x, bottom_right.y, bottom_right.z],
            [top_left.x, bottom_right.y, bottom_right.z],
            // Top face (higher Y)
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

        let mut vertices = Vec::with_capacity(24);
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

    fn floor_box() -> BoundingBox {
        BoundingBox {
            top_left: Point3::new(-0.5, 0.0, -0.5),
            bottom_right: Point3::new(Self::WIDTH as f32 - 0.5, -1.0, Self::HEIGHT as f32 - 0.5),
            collide_on_top: false,
        }
    }

    fn wall_box() -> BoundingBox {
        BoundingBox {
            top_left: Point3::new(-0.5, 1.0, -1.5),
            bottom_right: Point3::new(Self::WIDTH as f32 - 0.5, 0.0, -0.5),
            collide_on_top: true,
        }
    }
    fn wall_box_2() -> BoundingBox {
        BoundingBox {
            top_left: Point3::new(Self::WIDTH as f32 - 0.5, 1.0, -0.5),
            bottom_right: Point3::new(Self::WIDTH as f32 + 0.5, 0.0, Self::HEIGHT as f32 - 0.5),
            collide_on_top: true,
        }
    }
    fn floor_instance(device: &Device) -> (Buffer, u32) {
        let instances: Vec<RawInstance> = (0..(Self::WIDTH * Self::HEIGHT))
            .map(|pos| -> RawInstance {
                Instance {
                    position: Vector3::new(
                        (pos % Self::WIDTH) as f32,
                        0.0,
                        (pos / Self::HEIGHT) as f32,
                    ),
                    rotation: Matrix3::identity(),
                }
                .to_raw()
            })
            .collect();
        let num_instances = instances.len() as u32;
        (
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            num_instances,
        )
    }

    fn outer_wall_instance(device: &Device) -> (Buffer, u32) {
        let instances: Vec<RawInstance> = (0..Self::WIDTH)
            .map(|pos| -> RawInstance {
                Instance {
                    position: Vector3::new(pos as f32, 0.5, -0.5),
                    rotation: Matrix3::from(Rotation3::from_axis_angle(
                        &Vector3::y_axis(),
                        std::f32::consts::PI,
                    )),
                }
                .to_raw()
            })
            .collect();
        let num_instances = instances.len() as u32;

        (
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            num_instances,
        )
    }

    fn gen_mesh(vertices: &mut [Vertex], indices: &[u16], material: u32, device: &Device) -> Mesh {
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
            name: String::from("sample_mesh"),
            vertex_buffer,
            index_buffer,
            num_elements: num_indices,
            material,
        }
    }

    fn load_texture(
        filename: &str,
        normal_filename: &str,
        device: &Device,
        queue: &Queue,
        bind_group_layout: &BindGroupLayout,
    ) -> Material {
        let diffuse_texture = Texture::from_file(filename, device, queue, Some(filename));
        let normal_texture =
            Texture::from_file(normal_filename, device, queue, Some(normal_filename));
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
}
