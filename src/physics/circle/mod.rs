use crate::physics::{collider::Collider};

#[derive(Debug, Copy, Clone)]
pub struct Circle{
    pub radius: f32,
}

impl Collider for Circle{
    fn radius (&self) -> f32{
        self.radius
    }
    
}