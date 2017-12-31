
use ::Vertex;


use std::f32::consts::PI;


pub struct RenderBatch {
    pub(super) vertices: Vec<Vertex>,
    pub(super) indices: Vec<u32>,

    fill_color: [f32; 4]
}


impl RenderBatch {

    /// Create a new batch
    pub fn new() -> RenderBatch {
        RenderBatch {
            vertices: Vec::new(),
            indices: Vec::new(),

            fill_color: [1.0; 4]
        }
    }


    /// Start a new rendering operation
    pub fn begin(&mut self) {
        self.vertices.clear();
    }


    /// Set the current fill color
    pub fn set_fill_color(&mut self, color: [f32; 4]) {
        self.fill_color = color;
    }


    /// Draw a rectangle
    pub fn draw_rectangle(&mut self, position: [f32; 2], size: [f32; 2]) {
        let x: f32 = position[0];
        let y: f32 = position[1];
        let w: f32 = size[0];
        let h: f32 = size[1];

        let z = 0.0;

        let index_start: u32 = self.vertices.len() as u32;

        self.vertices.push(Vertex::new([x,     y,     z]).with_color(self.fill_color));
        self.vertices.push(Vertex::new([x + w, y,     z]).with_color(self.fill_color));
        self.vertices.push(Vertex::new([x + w, y + h, z]).with_color(self.fill_color));
        self.vertices.push(Vertex::new([x,     y + h, z]).with_color(self.fill_color));

        self.indices.push(index_start + 0);
        self.indices.push(index_start + 1);
        self.indices.push(index_start + 2);
        self.indices.push(index_start + 2);
        self.indices.push(index_start + 3);
        self.indices.push(index_start + 0);
    }


    /// Draw a circle with segments
    pub fn draw_circle_segments(&mut self, center: [f32; 2], radius: f32, segments: u32) {
        let x: f32 = center[0];
        let y: f32 = center[1];

        let z = 0.0;

        let index_start: u32 = self.vertices.len() as u32;

        // Add center vertex
        self.vertices.push(Vertex::new([x, y, z]).with_color(self.fill_color));

        // Add perimeter
        let delta_angle = 2.0 * PI / segments as f32;
        let mut angle: f32 = 0.0;

        for s in 0..segments {
            // Add perimeter vertices
            let (dy, dx) = angle.sin_cos();
            self.vertices.push(Vertex::new([x + radius * dx, y + radius * dy, z]).with_color(self.fill_color));

            // Add center
            self.indices.push(index_start);

            // Add perimeters
            self.indices.push(index_start + s + 1);
            self.indices.push(index_start + (s + 1) % segments + 1);

            // Increase angle
            angle += delta_angle;
        }
    }

    /// Draw a circle with automatic number of segments
    pub fn draw_circle(&mut self, center: [f32; 2], radius: f32) {
        self.draw_circle_segments(center, radius, 16);
    }
}
