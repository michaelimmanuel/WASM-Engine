#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
}

impl Shape {
    pub fn radius(&self) -> f32 {
        match self {
            Shape::Circle { radius } => *radius,
        }
    }
}