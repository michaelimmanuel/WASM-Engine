use crate::math::vec2::Vec2;
use crate::physics::body::Body;
use crate::physics::collider::Shape;

#[derive(Debug, Clone)]
pub struct Contact {
    pub normal: Vec2,
    pub penetration: f32,
    pub contact_point: Vec2,  
    pub body_a_index: usize,
    pub body_b_index: usize,
}

impl Contact {
    pub fn new(normal: Vec2, penetration: f32, contact_point: Vec2, body_a_index: usize, body_b_index: usize) -> Self {
        Self {
            normal,
            penetration,
            contact_point,
            body_a_index,
            body_b_index,
        }
    }
}

pub fn overlaps(a: &Body, b: &Body) -> bool {
    match(&a.shape, &b.shape)  {
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
            circle_vs_circle(a.position, *r1, b.position, *r2)
        }
        (Shape::Box { width: w1, height: h1 }, Shape::Box { width: w2, height: h2 }) => {
            // Use OBB collision if either box is rotated
            if a.rotation != 0.0 || b.rotation != 0.0 {
                obb_vs_obb(a.position, *w1, *h1, a.rotation, b.position, *w2, *h2, b.rotation)
            } else {
                aabb_vs_aabb(a.position, *w1, *h1, b.position, *w2, *h2)
            }
        }
        (Shape::Circle { radius }, Shape::Box { width, height }) => {
            if b.rotation != 0.0 {
                circle_vs_obb(a.position, *radius, b.position, *width, *height, b.rotation)
            } else {
                circle_vs_aabb(a.position, *radius, b.position, *width, *height)
            }
        }
        (Shape::Box { width, height }, Shape::Circle { radius }) => {
            if a.rotation != 0.0 {
                circle_vs_obb(b.position, *radius, a.position, *width, *height, a.rotation)
            } else {
                circle_vs_aabb(b.position, *radius, a.position, *width, *height)
            }
        }
    }
}

pub fn detect_collision(a: &Body, b: &Body, index_a: usize, index_b: usize) -> Option<Contact> {
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: r1 }, Shape::Circle { radius: r2 }) => {
            circle_collision(a.position, *r1, b.position, *r2, index_a, index_b)
        }
        (Shape::Box { width: w1, height: h1 }, Shape::Box { width: w2, height: h2 }) => {
            // Use OBB collision if either box is rotated
            if a.rotation != 0.0 || b.rotation != 0.0 {
                obb_collision(a.position, *w1, *h1, a.rotation, b.position, *w2, *h2, b.rotation, index_a, index_b)
            } else {
                aabb_collision(a.position, *w1, *h1, b.position, *w2, *h2, index_a, index_b)
            }
        }
        (Shape::Circle { radius }, Shape::Box { width, height }) => {
            if b.rotation != 0.0 {
                circle_obb_collision(a.position, *radius, b.position, *width, *height, b.rotation, index_a, index_b)
            } else {
                circle_aabb_collision(a.position, *radius, b.position, *width, *height, index_a, index_b)
            }
        }
        (Shape::Box { width, height }, Shape::Circle { radius }) => {
            // Box is A, Circle is B
            if a.rotation != 0.0 {
                circle_obb_collision(b.position, *radius, a.position, *width, *height, a.rotation, index_b, index_a)
                    .map(|contact| {
                        Contact::new(
                            contact.normal * -1.0,
                            contact.penetration,
                            contact.contact_point,
                            index_a,
                            index_b
                        )
                    })
            } else {
                circle_aabb_collision(b.position, *radius, a.position, *width, *height, index_b, index_a)
                    .map(|contact| {
                        Contact::new(
                            contact.normal * -1.0,
                            contact.penetration,
                            contact.contact_point,
                            index_a,
                            index_b
                        )
                    })
            }
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
    
    // Contact point is on the surface of circle A, along the collision normal
    let contact_point = pa + normal * r1;
    
    Some(Contact::new(normal, penetration, contact_point, index_a, index_b))
}

// AABB vs AABB collision detection
fn aabb_vs_aabb(pa: Vec2, w1: f32, h1: f32, pb: Vec2, w2: f32, h2: f32) -> bool {
    let half_w1 = w1 * 0.5;
    let half_h1 = h1 * 0.5;
    let half_w2 = w2 * 0.5;
    let half_h2 = h2 * 0.5;
    
    let dx = (pb.x - pa.x).abs();
    let dy = (pb.y - pa.y).abs();
    
    dx < (half_w1 + half_w2) && dy < (half_h1 + half_h2)
}

fn aabb_collision(pa: Vec2, w1: f32, h1: f32, pb: Vec2, w2: f32, h2: f32, 
                  index_a: usize, index_b: usize) -> Option<Contact> {
    let half_w1 = w1 * 0.5;
    let half_h1 = h1 * 0.5;
    let half_w2 = w2 * 0.5;
    let half_h2 = h2 * 0.5;
    
    let delta = pb - pa;
    let dx = delta.x.abs();
    let dy = delta.y.abs();
    
    // Check for overlap
    let overlap_x = (half_w1 + half_w2) - dx;
    let overlap_y = (half_h1 + half_h2) - dy;
    
    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return None;
    }
    
    // Find the axis of least penetration
    let (normal, penetration) = if overlap_x < overlap_y {
        // Collision on X axis
        let nx = if delta.x > 0.0 { 1.0 } else { -1.0 };
        (Vec2::new(nx, 0.0), overlap_x)
    } else {
        // Collision on Y axis
        let ny = if delta.y > 0.0 { 1.0 } else { -1.0 };
        (Vec2::new(0.0, ny), overlap_y)
    };
    
    // Contact point is at the edge of box A along the collision normal
    let contact_point = if overlap_x < overlap_y {
        // X-axis collision - contact on vertical edge of box A
        let edge_x = pa.x + normal.x * half_w1;
        let clamped_y = pb.y.clamp(pa.y - half_h1, pa.y + half_h1);
        Vec2::new(edge_x, clamped_y)
    } else {
        // Y-axis collision - contact on horizontal edge of box A
        let clamped_x = pb.x.clamp(pa.x - half_w1, pa.x + half_w1);
        let edge_y = pa.y + normal.y * half_h1;
        Vec2::new(clamped_x, edge_y)
    };
    
    Some(Contact::new(normal, penetration, contact_point, index_a, index_b))
}

// OBB vs OBB collision detection using Separating Axis Theorem (SAT)
fn obb_vs_obb(pa: Vec2, w1: f32, h1: f32, rot1: f32, 
              pb: Vec2, w2: f32, h2: f32, rot2: f32) -> bool {
    // Get the axes for both boxes
    let axes_a = get_obb_axes(rot1);
    let axes_b = get_obb_axes(rot2);
    
    // Test all 4 potential separating axes (2 per box)
    for axis in &[axes_a[0], axes_a[1], axes_b[0], axes_b[1]] {
        let (min_a, max_a) = project_obb(*axis, pa, w1, h1, rot1);
        let (min_b, max_b) = project_obb(*axis, pb, w2, h2, rot2);
        
        // Check for gap on this axis
        if max_a < min_b || max_b < min_a {
            return false; // Found separating axis - no collision
        }
    }
    
    true // No separating axis found - collision detected
}

fn obb_collision(pa: Vec2, w1: f32, h1: f32, rot1: f32,
                 pb: Vec2, w2: f32, h2: f32, rot2: f32,
                 index_a: usize, index_b: usize) -> Option<Contact> {
    // Get the axes for both boxes
    let axes_a = get_obb_axes(rot1);
    let axes_b = get_obb_axes(rot2);
    
    let mut min_overlap = f32::MAX;
    let mut collision_normal = Vec2::new(1.0, 0.0);
    
    // Test all 4 potential separating axes
    for axis in &[axes_a[0], axes_a[1], axes_b[0], axes_b[1]] {
        let (min_a, max_a) = project_obb(*axis, pa, w1, h1, rot1);
        let (min_b, max_b) = project_obb(*axis, pb, w2, h2, rot2);
        
        // Check for gap on this axis
        if max_a < min_b || max_b < min_a {
            return None; // Found separating axis - no collision
        }
        
        // Calculate overlap on this axis
        let overlap = (max_a.min(max_b) - min_a.max(min_b)).abs();
        
        if overlap < min_overlap {
            min_overlap = overlap;
            collision_normal = *axis;
            
            // Ensure normal points from A to B
            let delta = pb - pa;
            if delta.dot(*axis) < 0.0 {
                collision_normal = *axis * -1.0;
            }
        }
    }
    
    // Calculate contact point on the surface of box A
    // Project center of B onto the surface of A along the collision normal
    let half_w1 = w1 * 0.5;
    let half_h1 = h1 * 0.5;
    
    // Transform B's center to A's local space
    let cos1 = rot1.cos();
    let sin1 = rot1.sin();
    let delta = pb - pa;
    let local_b = Vec2::new(
        delta.x * cos1 + delta.y * sin1,
        -delta.x * sin1 + delta.y * cos1
    );
    
    // Clamp to box A's bounds
    let clamped_local = Vec2::new(
        local_b.x.clamp(-half_w1, half_w1),
        local_b.y.clamp(-half_h1, half_h1)
    );
    
    // Transform back to world space
    let contact_point = Vec2::new(
        clamped_local.x * cos1 - clamped_local.y * sin1 + pa.x,
        clamped_local.x * sin1 + clamped_local.y * cos1 + pa.y
    );
    
    Some(Contact::new(collision_normal, min_overlap, contact_point, index_a, index_b))
}

/// Get the two axes (normals) for an OBB based on its rotation
fn get_obb_axes(rotation: f32) -> [Vec2; 2] {
    let cos = rotation.cos();
    let sin = rotation.sin();
    [
        Vec2::new(cos, sin),           // X-axis of the rotated box
        Vec2::new(-sin, cos),          // Y-axis of the rotated box
    ]
}

/// Project an OBB onto an axis and return the min/max scalar values
fn project_obb(axis: Vec2, center: Vec2, width: f32, height: f32, rotation: f32) -> (f32, f32) {
    // Get the corner offsets in local space
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    
    // Get the rotated axes of the box
    let cos = rotation.cos();
    let sin = rotation.sin();
    let x_axis = Vec2::new(cos, sin);
    let y_axis = Vec2::new(-sin, cos);
    
    // Project the half-extents onto the test axis
    let proj_w = (x_axis.dot(axis) * half_w).abs();
    let proj_h = (y_axis.dot(axis) * half_h).abs();
    let radius = proj_w + proj_h;
    
    // Project center onto axis
    let center_proj = center.dot(axis);
    
    (center_proj - radius, center_proj + radius)
}

// Circle vs AABB collision detection
fn circle_vs_aabb(pc: Vec2, radius: f32, pb: Vec2, width: f32, height: f32) -> bool {
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    
    // Find closest point on AABB to circle center
    let closest_x = pc.x.clamp(pb.x - half_w, pb.x + half_w);
    let closest_y = pc.y.clamp(pb.y - half_h, pb.y + half_h);
    
    let dx = pc.x - closest_x;
    let dy = pc.y - closest_y;
    let dist_sq = dx * dx + dy * dy;
    
    dist_sq <= radius * radius
}

fn circle_aabb_collision(pc: Vec2, radius: f32, pb: Vec2, width: f32, height: f32,
                        circle_index: usize, box_index: usize) -> Option<Contact> {
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    
    // Find closest point on AABB to circle center
    let closest_x = pc.x.clamp(pb.x - half_w, pb.x + half_w);
    let closest_y = pc.y.clamp(pb.y - half_h, pb.y + half_h);
    let closest = Vec2::new(closest_x, closest_y);
    
    let delta = closest - pc;  // Points from circle center toward closest point on box
    let dist_sq = delta.x * delta.x + delta.y * delta.y;
    
    if dist_sq > radius * radius {
        return None;
    }
    
    let dist = dist_sq.sqrt();
    
    // Check if circle center is actually inside the box
    let inside_box = pc.x >= (pb.x - half_w) && pc.x <= (pb.x + half_w) &&
                     pc.y >= (pb.y - half_h) && pc.y <= (pb.y + half_h);
    
    // Normal should point from circle (A) toward box (B)
    let (normal, penetration) = if inside_box {
        // Circle center is inside the box - find which edge is closest
        let dx_right = (pb.x + half_w) - pc.x;
        let dx_left = pc.x - (pb.x - half_w);
        let dy_down = (pb.y + half_h) - pc.y;
        let dy_up = pc.y - (pb.y - half_h);
        
        // Find minimum distance to any edge
        let min_dist = dx_right.min(dx_left).min(dy_down).min(dy_up);
        
        // Normal points from circle toward the nearest edge (from A toward B)
        let normal = if min_dist == dx_right {
            Vec2::new(1.0, 0.0)  // Toward right edge
        } else if min_dist == dx_left {
            Vec2::new(-1.0, 0.0)  // Toward left edge
        } else if min_dist == dy_down {
            Vec2::new(0.0, 1.0)  // Toward bottom edge
        } else {
            Vec2::new(0.0, -1.0)  // Toward top edge
        };
        
        (normal, radius + min_dist)
    } else {
        // Circle center is outside the box
        if dist > 0.0001 {
            // Normal points from circle toward box (from A toward B)
            (delta.normalize(), radius - dist)
        } else {
            // Circle surface is exactly touching box edge
            // Determine which edge based on position
            let dx = pc.x - pb.x;
            let dy = pc.y - pb.y;
            
            let normal = if dx.abs() > dy.abs() {
                if dx > 0.0 { Vec2::new(-1.0, 0.0) } else { Vec2::new(1.0, 0.0) }
            } else {
                if dy > 0.0 { Vec2::new(0.0, -1.0) } else { Vec2::new(0.0, 1.0) }
            };
            
            (normal, radius)
        }
    };
    
    // Contact point is the closest point on the box surface
    let contact_point = Vec2::new(closest_x, closest_y);
    
    Some(Contact::new(normal, penetration, contact_point, circle_index, box_index))
}

// Circle vs OBB (Oriented Bounding Box) collision detection
fn circle_vs_obb(pc: Vec2, radius: f32, pb: Vec2, width: f32, height: f32, rotation: f32) -> bool {
    // Transform circle center to box's local space
    let delta = pc - pb;
    let local_circle = Vec2::new(
        delta.dot(Vec2::new(rotation.cos(), rotation.sin())),
        delta.dot(Vec2::new(-rotation.sin(), rotation.cos()))
    );
    
    // Now we have an AABB vs circle test in local space
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    
    let closest_x = local_circle.x.clamp(-half_w, half_w);
    let closest_y = local_circle.y.clamp(-half_h, half_h);
    
    let dx = local_circle.x - closest_x;
    let dy = local_circle.y - closest_y;
    let dist_sq = dx * dx + dy * dy;
    
    dist_sq <= radius * radius
}

fn circle_obb_collision(pc: Vec2, radius: f32, pb: Vec2, width: f32, height: f32, rotation: f32,
                        circle_index: usize, box_index: usize) -> Option<Contact> {
    // Transform circle center to box's local space
    let delta = pc - pb;
    let cos = rotation.cos();
    let sin = rotation.sin();
    
    let local_circle = Vec2::new(
        delta.dot(Vec2::new(cos, sin)),
        delta.dot(Vec2::new(-sin, cos))
    );
    
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    
    // Find closest point on box (in local space)
    let closest_x = local_circle.x.clamp(-half_w, half_w);
    let closest_y = local_circle.y.clamp(-half_h, half_h);
    let local_closest = Vec2::new(closest_x, closest_y);
    
    let local_delta = local_closest - local_circle;
    let dist_sq = local_delta.x * local_delta.x + local_delta.y * local_delta.y;
    
    if dist_sq > radius * radius {
        return None;
    }
    
    let dist = dist_sq.sqrt();
    
    // Check if circle center is inside the box
    let inside_box = local_circle.x >= -half_w && local_circle.x <= half_w &&
                     local_circle.y >= -half_h && local_circle.y <= half_h;
    
    let (local_normal, penetration) = if inside_box {
        // Circle center is inside - find closest edge
        let dx_right = half_w - local_circle.x;
        let dx_left = local_circle.x + half_w;
        let dy_down = half_h - local_circle.y;
        let dy_up = local_circle.y + half_h;
        
        let min_dist = dx_right.min(dx_left).min(dy_down).min(dy_up);
        
        let local_normal = if min_dist == dx_right {
            Vec2::new(1.0, 0.0)
        } else if min_dist == dx_left {
            Vec2::new(-1.0, 0.0)
        } else if min_dist == dy_down {
            Vec2::new(0.0, 1.0)
        } else {
            Vec2::new(0.0, -1.0)
        };
        
        (local_normal, radius + min_dist)
    } else {
        // Circle center is outside
        if dist > 0.0001 {
            (local_delta.normalize(), radius - dist)
        } else {
            // Edge case: circle exactly on box edge
            let local_normal = if local_circle.x.abs() > local_circle.y.abs() {
                if local_circle.x > 0.0 { Vec2::new(-1.0, 0.0) } else { Vec2::new(1.0, 0.0) }
            } else {
                if local_circle.y > 0.0 { Vec2::new(0.0, -1.0) } else { Vec2::new(0.0, 1.0) }
            };
            (local_normal, radius)
        }
    };
    
    // Transform normal back to world space
    let world_normal = Vec2::new(
        local_normal.x * cos - local_normal.y * sin,
        local_normal.x * sin + local_normal.y * cos
    );
    
    // Transform contact point back to world space
    let world_contact = Vec2::new(
        local_closest.x * cos - local_closest.y * sin + pb.x,
        local_closest.x * sin + local_closest.y * cos + pb.y
    );
    
    Some(Contact::new(world_normal, penetration, world_contact, circle_index, box_index))
}

pub fn resolve_collision(a: &mut Body, b: &mut Body, contact: &Contact) {
    // Skip if both bodies are static (infinite mass)
    if a.inv_mass == 0.0 && b.inv_mass == 0.0 {
        return;
    }

    // Calculate relative velocity at the contact point (includes angular component)
    let ra = contact.contact_point - a.position;
    let rb = contact.contact_point - b.position;
    
    // Velocity at contact point = linear_velocity + angular_velocity × r
    // In 2D: v = v_linear + ω × r, where ω × r = ω * perpendicular(r)
    let va = a.velocity + ra.perpendicular() * a.angular_velocity;
    let vb = b.velocity + rb.perpendicular() * b.angular_velocity;
    
    let rel_velocity = vb - va;
    let vel_along_normal = rel_velocity.dot(contact.normal);
    
    // Don't resolve if objects are separating
    if vel_along_normal > 0.0 {
        return;
    }
    
    let restitution = (a.restitution + b.restitution) * 0.5;
    
    // Calculate impulse magnitude with angular component
    // j = -(1 + e) * v_rel • n / (1/m_a + 1/m_b + (r_a × n)² / I_a + (r_b × n)² / I_b)
    let ra_cross_n = ra.cross(contact.normal);
    let rb_cross_n = rb.cross(contact.normal);
    
    let inv_mass_sum = a.inv_mass + b.inv_mass;
    let angular_factor = ra_cross_n * ra_cross_n * a.inv_moment_of_inertia +
                         rb_cross_n * rb_cross_n * b.inv_moment_of_inertia;
    
    let impulse_scalar = -(1.0 + restitution) * vel_along_normal / (inv_mass_sum + angular_factor);
    let impulse = contact.normal * impulse_scalar;
    
    // Apply impulses at contact points (generates both linear and angular motion)
    a.apply_impulse_at_point(impulse * -1.0, contact.contact_point);
    b.apply_impulse_at_point(impulse, contact.contact_point);
    
    // Dampen angular velocity on collision to improve stability
    // This prevents extreme spinning that can cause tunneling
    const ANGULAR_DAMPING: f32 = 0.9;
    a.angular_velocity *= ANGULAR_DAMPING;
    b.angular_velocity *= ANGULAR_DAMPING;
    
    // Positional correction (to prevent sinking)
    // Increased correction for better stability with rotation
    const PERCENT: f32 = 1.0;  // Full correction
    const SLOP: f32 = 0.01;    // Small tolerance
    
    let correction_magnitude = (contact.penetration - SLOP).max(0.0) / (a.inv_mass + b.inv_mass) * PERCENT;
    let correction = contact.normal * correction_magnitude;
    
    a.position = a.position - correction * a.inv_mass;
    b.position = b.position + correction * b.inv_mass;
    
    // If one body is static (wall), ensure dynamic body is fully pushed out
    if a.inv_mass == 0.0 && contact.penetration > SLOP {
        b.position = b.position + contact.normal * contact.penetration;
    } else if b.inv_mass == 0.0 && contact.penetration > SLOP {
        a.position = a.position - contact.normal * contact.penetration;
    }
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

