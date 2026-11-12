use crate::Buffer;

#[derive(Debug, Clone)]
pub struct GpuMesh {
    vertices: Vec<u32>,
    index_count: u32,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl GpuMesh {
    pub fn new(
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        vertices: Vec<u32>,
        index_count: u32,
    ) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            vertices,
            index_count,
        }
    }
}

impl GpuMesh {
    pub fn vertices(&self) -> &Vec<u32> {
        &self.vertices
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }
}
