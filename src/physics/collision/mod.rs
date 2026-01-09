use crate::math::vec2::Vec2;
use crate::physics::body::Body;
use crate::physics::collider::Shape;



pub fn overlaps(a: &Body, b: &Body) -> bool {
    match(&a.shape,&b.shape)  {
       (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
          
                circle_vs_circle(a.position, *r1, b.position, *r2)
            
       }
    }
}

fn circle_vs_circle(pa : Vec2, r1 : f32, pb : Vec2, r2 : f32) -> bool{
    let delta = pb - pa;
    let dist = delta.x * delta.x + delta.y * delta.y;
    let rsum = r1+r2;
    dist <= rsum*rsum
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

