use crate::math::vec2::Vec2;
use std::sync::Arc;
use crate::physics::circle::Circle;

use super::*;

#[test]
fn test_body_new() {
    let mass = 2.0;
    let position = Vec2::new(1.0, 2.0);
    let collider = Arc::new(Circle { radius: 1.0 });
    let body = Body::new(mass, position, collider);

    assert_eq!(body.mass, mass);
    assert_eq!(body.inv_mass, 0.5);
    assert_eq!(body.position, position);
    assert_eq!(body.velocity, Vec2::new(0.0, 0.0));
    assert_eq!(body.force, Vec2::new(0.0, 0.0));
}

#[test]
fn test_body_new_zero_mass() {
    let mass = 0.0;
    let position = Vec2::new(0.0, 0.0);
    let collider = Arc::new(Circle { radius: 1.0 });
    let body = Body::new(mass, position, collider);

    assert_eq!(body.mass, 0.0);
    assert_eq!(body.inv_mass, 0.0);
}

#[test]
fn test_apply_force() {
    let collider = Arc::new(Circle { radius: 1.0 });
    let mut body = Body::new(1.0, Vec2::new(0.0, 0.0), collider);
    let force = Vec2::new(3.0, 4.0);
    body.apply_force(force);

    assert_eq!(body.force, force);

    // Applying another force should accumulate
    let force2 = Vec2::new(1.0, 1.0);
    body.apply_force(force2);
    assert_eq!(body.force, Vec2::new(4.0, 5.0));
}

#[test]
fn test_integrate_updates_position_and_velocity() {
    let collider = Arc::new(Circle { radius: 1.0 });
    let mut body = Body::new(2.0, Vec2::new(0.0, 0.0), collider);
    let dt = 1.0;
    let initial_velocity = Vec2::new(0.0, 0.0);
    body.velocity = initial_velocity;

    // Apply a force before integration
    body.apply_force(Vec2::new(2.0, 0.0));
    body.integrate(dt);

    // After integration, force should be reset
    assert_eq!(body.force, Vec2::new(0.0, 0.0));

    // Gravity is applied internally: (0, 9.81) * mass = (0, 19.62)
    // Total force: (2.0, 0.0) + (0.0, 19.62) = (2.0, 19.62)
    // Acceleration: force / mass = (2.0, 19.62) / 2.0 = (1.0, 9.81)
    // Velocity after dt: v = v0 + a*dt = (0.0, 0.0) + (1.0, 9.81) = (1.0, 9.81)
    // Position after dt: p = p0 + v*dt = (0.0, 0.0) + (1.0, 9.81) = (1.0, 9.81)
    assert!((body.velocity.x - 1.0).abs() < 1e-5);
    assert!((body.velocity.y - 9.81).abs() < 1e-5);
    assert!((body.position.x - 1.0).abs() < 1e-5);
    assert!((body.position.y - 9.81).abs() < 1e-5);
}

#[test]
fn test_integrate_static_body() {
    let collider = Arc::new(Circle { radius: 1.0 });
    let mut body = Body::new(0.0, Vec2::new(5.0, 5.0), collider);
    body.apply_force(Vec2::new(10.0, 10.0));
    body.integrate(1.0);

    // Static body should not move or accumulate velocity
    assert_eq!(body.position, Vec2::new(5.0, 5.0));
    assert_eq!(body.velocity, Vec2::new(0.0, 0.0));
    // Force should remain unchanged since integrate returns early
    assert_eq!(body.force, Vec2::new(10.0, 10.0));
}
