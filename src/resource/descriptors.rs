use crate::resource::geometry::{IndexFormat, VertexBufferLayout};
use crate::resource::ids::TextureId;

/// Mesh vertex and index buffer data with layout metadata.
#[derive(Debug, Clone)]
pub struct MeshDescriptor {
    pub vertex_data: Vec<u8>,
    pub vertex_layout: VertexBufferLayout,
    pub index_data: Vec<u8>,
    pub index_format: IndexFormat,
    pub index_count: u32,
}

/// Supported texture pixel formats.
#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    Rgba8Unorm,
}

/// Face culling mode for rasterization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CullMode {
    None,
    Front,
    Back,
}

impl Default for CullMode {
    fn default() -> Self {
        Self::Back
    }
}

/// Texture image data and dimensions.
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

/// Shader pipeline and raster state for a draw material.
#[derive(Debug, Clone)]
pub struct MaterialDescriptor {
    pub vertex_shader_spv: Vec<u8>,
    pub vertex_entry: String,
    pub fragment_shader_spv: Vec<u8>,
    pub fragment_entry: String,
    pub enable_depth: bool,
    pub cull_mode: CullMode,
    pub texture: Option<TextureId>,
}

impl MaterialDescriptor {
    /// Creates a material with default [CullMode] and the given shaders.
    pub fn new(
        vertex_shader_spv: Vec<u8>,
        vertex_entry: String,
        fragment_shader_spv: Vec<u8>,
        fragment_entry: String,
        enable_depth: bool,
        texture: Option<TextureId>,
    ) -> Self {
        Self {
            vertex_shader_spv,
            vertex_entry,
            fragment_shader_spv,
            fragment_entry,
            enable_depth,
            cull_mode: CullMode::default(),
            texture,
        }
    }

    /// Sets the vertex shader entry point name.
    pub fn with_vertex_entry(mut self, entry: impl Into<String>) -> Self {
        self.vertex_entry = entry.into();
        self
    }

    /// Sets the fragment shader entry point name.
    pub fn with_fragment_entry(mut self, entry: impl Into<String>) -> Self {
        self.fragment_entry = entry.into();
        self
    }

    /// Sets the face culling mode.
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }
}
