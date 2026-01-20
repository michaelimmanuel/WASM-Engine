use crate::math::vec2::Vec2;
use crate::physics::body::Body;
use crate::physics::collider::Shape;

#[derive(Debug, Clone)]
pub struct Contact {
    pub normal: Vec2,
    pub penetration: f32,
    pub body_a_index: usize,
    pub body_b_index: usize,
}

impl Contact {
    pub fn new(normal: Vec2, penetration: f32, body_a_index: usize, body_b_index: usize) -> Self {
        Self {
            normal,
            penetration,
            body_a_index,
            body_b_index,
        }
    }
}

pub fn overlaps(a: &Body, b: &Body) -> bool {
    match(&a.shape,&b.shape)  {
       (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
          
                circle_vs_circle(a.position, *r1, b.position, *r2)
            
       }
    }
}

pub fn detect_collision(a: &Body, b: &Body, index_a: usize, index_b: usize) -> Option<Contact> {
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
            circle_collision(a.position, *r1, b.position, *r2, index_a, index_b)
        }
    }
}

fn circle_vs_circle(pa : Vec2, r1 : f32, pb : Vec2, r2 : f32) -> bool{
    let delta = pb - pa;
    let dist = delta.x * delta.x + delta.y * delta.y;
    let rsum = r1+r2;
    dist <= rsum*rsum
}

fn circle_collision(pa: Vec2, r1: f32, pb: Vec2, r2: f32, index_a: usize, index_b: usize) -> Option<Contact> {
    let delta = pb - pa;
    let dist_sq = delta.x * delta.x + delta.y * delta.y;
    let rsum = r1 + r2;
    
    if dist_sq > rsum * rsum {
        return None;
    }
    
    let dist = dist_sq.sqrt();
    

    let normal = if dist > 0.0001 {
        delta / dist
    } else {
        Vec2::new(1.0, 0.0)
    };
    
    let penetration = rsum - dist;
    
    Some(Contact::new(normal, penetration, index_a, index_b))
}

pub fn resolve_collision(a: &mut Body, b: &mut Body, contact: &Contact) {

    let rel_velocity = b.velocity - a.velocity;
    let vel_along_normal = rel_velocity.dot(contact.normal);
    
    if vel_along_normal > 0.0 {
        return;
    }
    
    let restitution = (a.restitution + b.restitution) * 0.5;
    
    let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (a.inv_mass + b.inv_mass);
    
    let impulse = contact.normal * impulse_scalar;
    a.apply_impulse(impulse * -1.0);
    b.apply_impulse(impulse);
    

    const PERCENT: f32 = 0.4; 
    const SLOP: f32 = 0.01; 
    
    let correction_magnitude = (contact.penetration - SLOP).max(0.0) / (a.inv_mass + b.inv_mass) * PERCENT;
    let correction = contact.normal * correction_magnitude;
    
    a.position = a.position - correction * a.inv_mass;
    b.position = b.position + correction * b.inv_mass;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vec2::Vec2;
    use crate::physics::collider::Shape;
    use crate::physics::body::Body;

    #[test]
    fn circles_overlap_when_close() {
        let a = Body::new(1.0, Vec2::new(0.0, 0.0), Shape::Circle { radius: 1.0 });
        let b = Body::new(1.0, Vec2::new(1.0, 0.0), Shape::Circle { radius: 1.0 });
        assert!(overlaps(&a, &b));
    }

    #[test]
    fn circles_do_not_overlap_when_far() {
        let a = Body::new(1.0, Vec2::new(0.0, 0.0), Shape::Circle { radius: 1.0 });
        let b = Body::new(1.0, Vec2::new(3.0, 0.0), Shape::Circle { radius: 1.0 });
        assert!(!overlaps(&a, &b));
    }
}

