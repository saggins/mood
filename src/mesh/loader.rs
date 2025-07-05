use super::{Mesh, Vertex};
use nalgebra::{Point3, Vector3};

pub struct MeshLoader;

impl MeshLoader {
    pub fn temp_load_mesh() -> Mesh {
        // Define the cube vertex positions and colors only
        let positions = vec![
            Point3::new(-0.5, -0.5, 0.5),  // 0
            Point3::new(0.5, -0.5, 0.5),   // 1
            Point3::new(0.5, 0.5, 0.5),    // 2
            Point3::new(-0.5, 0.5, 0.5),   // 3
            Point3::new(-0.5, -0.5, -0.5), // 4
            Point3::new(0.5, -0.5, -0.5),  // 5
            Point3::new(0.5, 0.5, -0.5),   // 6
            Point3::new(-0.5, 0.5, -0.5),  // 7
        ];

        let colors = vec![
            [1.0, 0.0, 0.0], // red
            [1.0, 0.0, 0.0], // green
            [1.0, 0.0, 0.0], // blue
            [1.0, 0.0, 0.0], // yellow
            [1.0, 0.0, 0.0], // magenta
            [1.0, 0.0, 0.0], // cyan
            [1.0, 0.0, 0.0], // white
            [1.0, 0.0, 0.0], // black
        ];

        let indices: Vec<u16> = vec![
            // front
            0, 1, 2, 2, 3, 0, // right
            1, 5, 6, 6, 2, 1, // back
            5, 4, 7, 7, 6, 5, // left
            4, 0, 3, 3, 7, 4, // top
            3, 2, 6, 6, 7, 3, // bottom
            4, 5, 1, 1, 0, 4,
        ];

        // Initialize normals
        let mut normals = vec![Vector3::zeros(); positions.len()];

        // Accumulate face normals
        for triangle in indices.chunks(3) {
            let i0 = triangle[0] as usize;
            let i1 = triangle[1] as usize;
            let i2 = triangle[2] as usize;

            let p0 = positions[i0];
            let p1 = positions[i1];
            let p2 = positions[i2];

            let edge1 = p1 - p0;
            let edge2 = p2 - p0;
            let face_normal = edge1.cross(&edge2).normalize();

            normals[i0] += face_normal;
            normals[i1] += face_normal;
            normals[i2] += face_normal;
        }

        // Normalize all vertex normals
        let normals: Vec<[f32; 3]> = normals.into_iter().map(|n| n.normalize().into()).collect();

        // Build final vertices
        let verticies: Vec<Vertex> = positions
            .into_iter()
            .zip(colors)
            .zip(normals)
            .map(|((pos, color), normal)| Vertex {
                position: [pos.x, pos.y, pos.z],
                color,
                normal,
            })
            .collect();

        Mesh { verticies, indices }
    }
}
