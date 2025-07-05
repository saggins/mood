use super::{Mesh, Vertex};
use nalgebra::{Point3, Vector3};

pub struct MeshLoader;

impl MeshLoader {
    pub fn temp_load_mesh() -> Mesh {
        let vertices = vec![
            // Front face (+Z)
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (-Z)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.0, 1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Left face (-X)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.0, 0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            // Right face (+X)
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            // Top face (+Y)
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1.0, 0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            // Bottom face (-Y)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.0, 1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];

        let indices: Vec<u16> = vec![
            // Front face
            0, 1, 2, 2, 3, 0, // Back face
            4, 5, 6, 6, 7, 4, // Left face
            8, 9, 10, 10, 11, 8, // Right face
            12, 13, 14, 14, 15, 12, // Top face
            16, 17, 18, 18, 19, 16, // Bottom face
            20, 21, 22, 22, 23, 20,
        ];

        Mesh {
            verticies: vertices,
            indices,
        }
    }
}
