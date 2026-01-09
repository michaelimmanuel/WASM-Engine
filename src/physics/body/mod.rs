use crate::{
    math::vec2::Vec2, 
    physics::collider::Shape
};

pub struct Body {
    
    pub mass: f32,
    pub inv_mass: f32,
    pub velocity: Vec2,
    pub position: Vec2,
    pub force: Vec2,

    pub shape: Shape,
}

impl Body {

    const GRAVITY: Vec2 = Vec2 { x: 0.0, y: 9.81 };

    pub fn new(mass: f32, position: Vec2, shape: Shape) -> Self {
        let inv_mass = if mass != 0.0 { 1.0 / mass } else { 0.0 };
        Self {
            mass,
            inv_mass,
            velocity: Vec2::new(0.0, 0.0),
            position,
            force: Vec2::new(0.0, 0.0),
            shape,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.force = self.force + force;
    }

    pub fn integrate(&mut self, dt: f32) {
        if self.inv_mass == 0.0 {
            return;
        }

        // self.apply_force(Self::GRAVITY * self.mass);

        let acceleration = self.force * self.inv_mass;
        self.velocity = self.velocity + acceleration * dt;

        self.position = self.position + self.velocity * dt;

        self.force = Vec2::new(0.0, 0.0);
    }

}

#[cfg(test)]
#[path = "tests/body_tests.rs"]
mod body_tests;