use wasm_bindgen::prelude::*;
use crate::math::vec2::Vec2;
use crate::physics::body::Body;
use crate::physics::collider::Shape;
use crate::physics::collision::{overlaps};

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
        let shape = Shape::Circle { radius: 1.0 }; // Default radius
        let body = Body::new(mass, position, shape);
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

        let len = self.bodies.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = self.bodies.split_at_mut(j);
                let a = &left[i];
                let b = &right[0];

                if overlaps(a,b){
                    println!("collide")
                }
            }
        }
    }
    
    #[wasm_bindgen]
    pub fn collides(&self, i: usize, j: usize) -> bool {
        let a = self.bodies.get(i);
        let b = self.bodies.get(j);
        match (a, b) {
            (Some(a_body), Some(b_body)) => overlaps(a_body, b_body),
            _ => false,
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