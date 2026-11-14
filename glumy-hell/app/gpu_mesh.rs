use bytemuck::cast_slice;

use crate::{Buffer, BufferDescriptor, BufferUsages, Mesh};

#[derive(Debug, Clone)]
pub struct GpuMesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn new(mesh: &Mesh) -> Self {
        let vertex_buffer = Buffer::new(BufferDescriptor {
            label: "Vertex Buffer Descriptor",
            content: cast_slice(mesh.vertices()),
            usages: BufferUsages::VERTEX,
        });

        let index_buffer = Buffer::new(BufferDescriptor {
            label: "Index Buffer Descriptor",
            content: cast_slice(mesh.vertices()),
            usages: BufferUsages::INDEX,
        });

        let index_count = mesh.indices().len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

impl GpuMesh {
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
