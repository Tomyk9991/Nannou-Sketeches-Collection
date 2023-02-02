use nannou::geom::Rect;
use nannou::rand::random_f32;
use noise::{NoiseFn, Perlin};
use crate::geometry::grid::{Grid, GridIterator};

pub struct FlowField {
    grid: Grid,
    sampler: Perlin,
    scale: (f32, f32)
}

impl FlowField {
    pub fn new(window: &Rect) -> Self {
        let margin = 5.0;

        Self {
            grid: Grid::new(window.left() + (margin * 5.0), window.right(), window.bottom() - margin, window.top() + margin, 30, 30),
            sampler: Perlin::new((random_f32() * 500.0) as u32),
            scale: (window.w() * 0.23, window.h() * 1.0),
        }
    }

    pub fn scale(&self, x: f32, y: f32) -> (f32, f32) {
        (x / self.scale.0, y / self.scale.1)
    }

    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let value = self.sampler.get([x as f64, y as f64]);
        return ((value + 1.0) / 2.0) as f32; // normalize
    }
}

impl IntoIterator for &FlowField {
    type Item = (f32, f32);
    type IntoIter = GridIterator;

    fn into_iter(self) -> Self::IntoIter {
        Grid::iter(&self.grid)
    }
}

impl Iterator for FlowField {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        self.grid.iter().next()
    }
}