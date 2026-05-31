use crate::resource::descriptors::{MaterialDescriptor, MeshDescriptor, TextureDescriptor};
use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::{MaterialId, MeshId, TextureId};

/// Resource creation entry for a batch operation.
#[derive(Debug, Clone)]
pub enum ResourceCreateDescriptor {
    Mesh(MeshDescriptor),
    Texture(TextureDescriptor),
    Material(MaterialDescriptor),
}

/// Batch of resources to create in one submission.
#[derive(Debug, Clone)]
pub struct ResourceBatchCreate {
    pub resources: Vec<ResourceCreateDescriptor>,
}

impl ResourceBatchCreate {
    /// Creates a batch from the given resource descriptors.
    pub fn new(resources: Vec<ResourceCreateDescriptor>) -> Self {
        Self { resources }
    }
}

/// Handle returned for a created resource.
#[derive(Debug, Clone, Copy)]
pub enum ResourceHandle {
    Mesh(MeshId),
    Texture(TextureId),
    Material(MaterialId),
}

/// Handles produced by a resource creation batch.
#[derive(Debug, Clone)]
pub struct CreatedResources {
    pub handles: Vec<ResourceHandle>,
}

/// Partial update to an existing resource.
#[derive(Debug, Clone)]
pub enum ResourceUpdateDescriptor {
    Mesh {
        id: MeshId,
        vertex_data: Vec<u8>,
        vertex_layout: VertexBufferLayout,
        index_data: Vec<u8>,
        index_format: IndexFormat,
        index_count: u32,
    },
    Texture {
        id: TextureId,
        data: Vec<u8>,
    },
    Material {
        id: MaterialId,
        enable_depth: Option<bool>,
        /// `Some(None)` clears the bound texture; `None` leaves it unchanged.
        texture: Option<Option<TextureId>>,
    },
}

/// Batch of resource updates to apply in one submission.
#[derive(Debug, Clone)]
pub struct ResourceBatchUpdate {
    pub updates: Vec<ResourceUpdateDescriptor>,
}

impl ResourceBatchUpdate {
    /// Creates a batch from the given update descriptors.
    pub fn new(updates: Vec<ResourceUpdateDescriptor>) -> Self {
        Self { updates }
    }
}
