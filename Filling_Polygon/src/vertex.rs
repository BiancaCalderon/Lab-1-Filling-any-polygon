use nalgebra_glm::Vec3;

pub type Vertex = Vec3;

pub trait VertexExt {
    fn new(x: f32, y: f32, z: f32) -> Self;
}

impl VertexExt for Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z)
    }
}
