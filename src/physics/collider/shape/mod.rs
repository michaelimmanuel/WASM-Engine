#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Circle { radius: f32 },
    Box { width: f32, height: f32 },
}

impl Shape {
    pub fn radius(&self) -> f32 {
        match self {
            Shape::Circle { radius } => *radius,
            Shape::Box { .. } => 0.0,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            Shape::Circle { radius } => radius * 2.0,
            Shape::Box { width, .. } => *width,
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Shape::Circle { radius } => radius * 2.0,
            Shape::Box { height, .. } => *height,
        }
    }
}