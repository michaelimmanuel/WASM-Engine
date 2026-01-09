use wasm_bindgen::prelude::*;
use crate::math::vec2::Vec2;
use crate::physics::body::Body;

#[wasm_bindgen]
pub struct World {
    bodies: Vec<Body>,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        World {
            bodies: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn create_body(&mut self, mass: f32, x: f32, y: f32) -> usize {
        let position = Vec2::new(x, y);
        let body = Body::new(mass, position);
        let index = self.bodies.len();
        self.bodies.push(body);
        index
    }

    #[wasm_bindgen]
    pub fn get_body_position_x(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.position.x).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn get_body_position_y(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.position.y).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn step(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.integrate(dt);
        }
    }

    #[wasm_bindgen]
    pub fn bodies_count(&self) -> usize {
        self.bodies.len()
    }
}

#[cfg(test)]
#[path = "tests/world_tests.rs"]
mod world_tests;