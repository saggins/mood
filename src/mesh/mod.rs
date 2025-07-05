pub mod loader;
use super::renderer::vertex::Vertex;

pub struct Mesh {
    pub verticies: Vec<Vertex>,
    pub indices: Vec<u16>,
}
