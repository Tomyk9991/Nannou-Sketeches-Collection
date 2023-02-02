use std::f32::consts::PI;
use nannou::color::Srgba;
use nannou::geom::{Point2, Rect};
use nannou::prelude::random_f32;
use crate::flowfield::FlowField;

#[derive(Clone)]
pub struct ParticleSegment {
    pub position: Point2,
    pub color: Srgba<f64>,
    pub time_left: f32
}


pub struct Particle {
    pub pos: Point2,
    pub vel: Point2,
    pub particle_trail: Vec<ParticleSegment>
}

pub struct UpdateResult {
    pub previous_position: Point2,
    pub jumped: bool
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Particle {
            pos: Point2::new(x, y),
            vel: Point2::new(random_f32() * 400.0, random_f32() * 400.0),
            particle_trail: vec![]
        }
    }

    /// returns the last position
    pub fn update(&mut self, flow_field: &FlowField, bounds: &Rect, delta_time: f64) -> UpdateResult {
        let last_position = self.pos.clone();

        let (sample_x, sample_y) = flow_field.scale(self.pos.x, self.pos.y);
        let normalized_value = flow_field.sample(sample_x, sample_y);
        let angle = normalized_value * 2.0 * PI;
        let acceleration = Point2::new(angle.cos(), angle.sin()) * 10.0;
        self.vel += acceleration * 0.01;
        self.vel = self.vel.clamp_length_max(10.0);

        self.pos += (self.vel * delta_time as f32) * 10.0;

        let mut jumped = false;
        if self.pos.x < bounds.left() {
            self.pos.x = bounds.right();
            jumped = true;
        } else if self.pos.x > bounds.right() {
            self.pos.x = bounds.left();
            jumped = true;
        } else if self.pos.y > bounds.top() {
            self.pos.y = bounds.bottom();
            jumped = true;
        } else if self.pos.y < bounds.bottom() {
            self.pos.y = bounds.top();
            jumped = true;
        }

        return UpdateResult {
            previous_position: last_position,
            jumped
        };
    }
}