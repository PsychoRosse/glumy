use crate::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    const FRONT_NORMAL: [f32; 3] = [0.0, 0.0, 1.0];
    const BACK_NORMAL: [f32; 3] = [0.0, 0.0, -1.0];
    const TOP_NORMAL: [f32; 3] = [0.0, 1.0, 0.0];
    const BOTTOM_NORMAL: [f32; 3] = [0.0, -1.0, 0.0];
    const RIGHT_NORMAL: [f32; 3] = [1.0, 0.0, 0.0];
    const LEFT_NORMAL: [f32; 3] = [-1.0, 0.0, 0.0];
    const FRONT_UV: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
}

impl Default for Mesh {
    fn default() -> Self {
        Self::cube()
    }
}

impl Mesh {
    pub const fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}

impl Mesh {
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    pub const fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub const fn index_count(&self) -> usize {
        self.indices.len()
    }
}

impl Mesh {
    pub fn triangle_2d() -> Self {
        Mesh::new(
            vec![
                Vertex::new([-0.5, -0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[0]),
                Vertex::new([0.5, -0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[1]),
                Vertex::new([0.0, 0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[2]),
            ],
            vec![0, 1, 2],
        )
    }

    pub fn quad() -> Self {
        Mesh::new(
            vec![
                Vertex::new([-0.5, -0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[0]),
                Vertex::new([0.5, -0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[1]),
                Vertex::new([0.5, 0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[2]),
                Vertex::new([-0.5, 0.5, 0.0], Self::FRONT_NORMAL, Self::FRONT_UV[3]),
            ],
            vec![0, 1, 2, 2, 3, 0],
        )
    }

    pub fn cube() -> Self {
        let vertices = vec![
            Vertex::new([-0.5, -0.5, 0.5], Self::FRONT_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([0.5, -0.5, 0.5], Self::FRONT_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([0.5, 0.5, 0.5], Self::FRONT_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([-0.5, 0.5, 0.5], Self::FRONT_NORMAL, Self::FRONT_UV[3]),
            Vertex::new([0.5, -0.5, -0.5], Self::BACK_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([-0.5, -0.5, -0.5], Self::BACK_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([-0.5, 0.5, -0.5], Self::BACK_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([0.5, 0.5, -0.5], Self::BACK_NORMAL, Self::FRONT_UV[3]),
            Vertex::new([-0.5, 0.5, 0.5], Self::TOP_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([0.5, 0.5, 0.5], Self::TOP_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([0.5, 0.5, -0.5], Self::TOP_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([-0.5, 0.5, -0.5], Self::TOP_NORMAL, Self::FRONT_UV[3]),
            Vertex::new([-0.5, -0.5, -0.5], Self::BOTTOM_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([0.5, -0.5, -0.5], Self::BOTTOM_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([0.5, -0.5, 0.5], Self::BOTTOM_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([-0.5, -0.5, 0.5], Self::BOTTOM_NORMAL, Self::FRONT_UV[3]),
            Vertex::new([0.5, -0.5, 0.5], Self::RIGHT_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([0.5, -0.5, -0.5], Self::RIGHT_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([0.5, 0.5, -0.5], Self::RIGHT_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([0.5, 0.5, 0.5], Self::RIGHT_NORMAL, Self::FRONT_UV[3]),
            Vertex::new([-0.5, -0.5, -0.5], Self::LEFT_NORMAL, Self::FRONT_UV[0]),
            Vertex::new([-0.5, -0.5, 0.5], Self::LEFT_NORMAL, Self::FRONT_UV[1]),
            Vertex::new([-0.5, 0.5, 0.5], Self::LEFT_NORMAL, Self::FRONT_UV[2]),
            Vertex::new([-0.5, 0.5, -0.5], Self::LEFT_NORMAL, Self::FRONT_UV[3]),
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16,
            17, 18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
        ];

        Self::new(vertices, indices)
    }
}
