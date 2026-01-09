use crate::math::vec2::Vec2;
use crate::physics::body::Body;

pub struct World {
    bodies: Vec<Body>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn create_body(&mut self, mass: f32, position: Vec2) -> usize {
        let body = Body::new(mass, position);
        let index = self.bodies.len();
        self.bodies.push(body);
        index
    }

    pub fn get_body(&self, index: usize) -> Option<&Body> {
        self.bodies.get(index)
    }

    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut Body> {
        self.bodies.get_mut(index)
    }

    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.integrate(dt);
        }
    }

    pub fn bodies_count(&self) -> usize {
        self.bodies.len()
    }
}

#[cfg(test)]
#[path = "tests/world_tests.rs"]
mod world_tests;