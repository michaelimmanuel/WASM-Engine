use wasm_bindgen::prelude::*;
use crate::math::vec2::Vec2;
use crate::physics::body::Body;
use crate::physics::collider::Shape;
use crate::physics::collision::{overlaps, detect_collision, resolve_collision};

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
    pub fn create_circle(&mut self, mass: f32, x: f32, y: f32, radius: f32) -> usize {
        let position = Vec2::new(x, y);
        let shape = Shape::Circle { radius };
        let body = Body::new(mass, position, shape);
        let index = self.bodies.len();
        self.bodies.push(body);
        index
    }

    #[wasm_bindgen]
    pub fn create_box(&mut self, mass: f32, x: f32, y: f32, width: f32, height: f32) -> usize {
        let position = Vec2::new(x, y);
        let shape = Shape::Box { width, height };
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
    pub fn set_body_position(&mut self, index: usize, x: f32, y: f32) {
        if let Some(body) = self.bodies.get_mut(index) {
            body.position.x = x;
            body.position.y = y;
        }
    }

  #[wasm_bindgen]
    pub fn step(&mut self, dt: f32) {
    
        for body in &mut self.bodies {
            body.integrate(dt);
        }

        // Resolve collisions multiple times to prevent clipping
        for _ in 0..4 {
            self.resolve_collisions();
        }
    }

    #[wasm_bindgen]
    pub fn resolve_collisions(&mut self) {
        let mut contacts = Vec::new();
        let len = self.bodies.len();
        for i in 0..len {
            for j in (i + 1)..len {

                let a = &self.bodies[i];
                let b = &self.bodies[j];

                if let Some(contact) = detect_collision(a, b, i, j) {
                    contacts.push(contact);
                }
            }
        }

        // Resolve all collisions
        for contact in &contacts {
            let i = contact.body_a_index;
            let j = contact.body_b_index;
            
            // Split borrow: get mutable references to both bodies
            let (left, right) = self.bodies.split_at_mut(j);
            let body_a = &mut left[i];
            let body_b = &mut right[0];
            
            resolve_collision(body_a, body_b, contact);
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
    pub fn get_collision_penetration(&self, i: usize, j: usize) -> f32 {
        let a = self.bodies.get(i);
        let b = self.bodies.get(j);
        match (a, b) {
            (Some(a_body), Some(b_body)) => {
                if let Some(contact) = detect_collision(a_body, b_body, i, j) {
                    contact.penetration
                } else {
                    0.0
                }
            },
            _ => 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn get_collision_normal_x(&self, i: usize, j: usize) -> f32 {
        let a = self.bodies.get(i);
        let b = self.bodies.get(j);
        match (a, b) {
            (Some(a_body), Some(b_body)) => {
                if let Some(contact) = detect_collision(a_body, b_body, i, j) {
                    contact.normal.x
                } else {
                    0.0
                }
            },
            _ => 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn get_collision_normal_y(&self, i: usize, j: usize) -> f32 {
        let a = self.bodies.get(i);
        let b = self.bodies.get(j);
        match (a, b) {
            (Some(a_body), Some(b_body)) => {
                if let Some(contact) = detect_collision(a_body, b_body, i, j) {
                    contact.normal.y
                } else {
                    0.0
                }
            },
            _ => 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn bodies_count(&self) -> usize {
        self.bodies.len()
    }

    #[wasm_bindgen]
    pub fn set_body_gravity(&mut self, index: usize, enabled: bool) {
        if let Some(body) = self.bodies.get_mut(index) {
            body.set_gravity_enabled(enabled);
        }
    }

    #[wasm_bindgen]
    pub fn set_body_restitution(&mut self, index: usize, restitution: f32) {
        if let Some(body) = self.bodies.get_mut(index) {
            body.set_restitution(restitution);
        }
    }

    #[wasm_bindgen]
    pub fn set_body_velocity(&mut self, index: usize, vx: f32, vy: f32) {
        if let Some(body) = self.bodies.get_mut(index) {
            body.velocity = Vec2::new(vx, vy);
        }
    }

    #[wasm_bindgen]
    pub fn get_body_velocity_x(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.velocity.x).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn get_body_velocity_y(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.velocity.y).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn is_body_circle(&self, index: usize) -> bool {
        self.bodies.get(index).map(|body| {
            matches!(body.shape, Shape::Circle { .. })
        }).unwrap_or(false)
    }

    #[wasm_bindgen]
    pub fn is_body_box(&self, index: usize) -> bool {
        self.bodies.get(index).map(|body| {
            matches!(body.shape, Shape::Box { .. })
        }).unwrap_or(false)
    }

    #[wasm_bindgen]
    pub fn get_body_width(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.shape.width()).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn get_body_height(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.shape.height()).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn is_body_static(&self, index: usize) -> bool {
        self.bodies.get(index).map(|body| body.inv_mass == 0.0).unwrap_or(false)
    }

    #[wasm_bindgen]
    pub fn get_body_rotation(&self, index: usize) -> f32 {
        self.bodies.get(index).map(|body| body.rotation).unwrap_or(0.0)
    }

    #[wasm_bindgen]
    pub fn set_body_rotation(&mut self, index: usize, rotation: f32) {
        if let Some(body) = self.bodies.get_mut(index) {
            body.rotation = rotation;
        }
    }
}

#[cfg(test)]
#[path = "tests/world_tests.rs"]
mod world_tests;