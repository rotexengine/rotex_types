/// Render pass configuration within a [FrameDescriptor].
#[derive(Debug, Clone)]
pub struct PassDescriptor {
    pub name: String,
    pub clear_color: [f32; 4],
    pub clear_depth: Option<f32>,
    pub instance_indices: Vec<usize>,
}

impl PassDescriptor {
    /// Creates a pass named `name` with default clear values and no instances.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            clear_color: [0.04, 0.05, 0.08, 1.0],
            clear_depth: None,
            instance_indices: Vec::new(),
        }
    }

    /// Sets the color attachment clear value.
    pub fn with_clear_color(mut self, clear_color: [f32; 4]) -> Self {
        self.clear_color = clear_color;
        self
    }

    /// Sets the depth attachment clear value, or `None` to skip depth clearing.
    pub fn with_clear_depth(mut self, clear_depth: Option<f32>) -> Self {
        self.clear_depth = clear_depth;
        self
    }

    /// Sets which scene instance indices this pass draws.
    pub fn with_instance_indices(mut self, instance_indices: Vec<usize>) -> Self {
        self.instance_indices = instance_indices;
        self
    }
}

/// Ordered render passes for one frame.
#[derive(Debug, Clone)]
pub struct FrameDescriptor {
    pub passes: Vec<PassDescriptor>,
}

impl FrameDescriptor {
    /// Creates a frame from the given passes.
    pub fn new(passes: Vec<PassDescriptor>) -> Self {
        Self { passes }
    }

    /// Creates a frame with a single `"main"` pass.
    pub fn single_pass(clear_color: [f32; 4], clear_depth: Option<f32>) -> Self {
        Self {
            passes: vec![
                PassDescriptor::new("main")
                    .with_clear_color(clear_color)
                    .with_clear_depth(clear_depth),
            ],
        }
    }
}
