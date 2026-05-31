/// Vertex attribute component format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,
    Uint32,
}

impl VertexFormat {
    /// Size of one attribute value in bytes.
    pub fn size(&self) -> u64 {
        match self {
            Self::Float32 => 4,
            Self::Float32x2 => 8,
            Self::Float32x3 => 12,
            Self::Float32x4 => 16,
            Self::Uint32 => 4,
        }
    }
}

/// Single vertex input attribute binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexAttribute {
    pub location: u32,
    pub format: VertexFormat,
    pub offset: u64,
}

/// Vertex buffer layout describing stride and attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub attributes: Vec<VertexAttribute>,
}

/// Index buffer element format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}
