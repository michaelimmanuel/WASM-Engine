use crate::math::vec2::Vec2;
use crate::physics::world::World;

#[test]
fn test_world_creation() {
    let world = World::new();
    assert_eq!(world.bodies_count(), 0);
}

#[test]
fn test_add_body() {
    let mut world = World::new();
    let body = crate::physics::body::Body::new(1.0, Vec2::new(0.0, 0.0));
    world.add_body(body);
    assert_eq!(world.bodies_count(), 1);
}

#[test]
fn test_create_body() {
    let mut world = World::new();
    let index = world.create_body(2.0, Vec2::new(1.0, 2.0));
    assert_eq!(index, 0);
    assert_eq!(world.bodies_count(), 1);

    if let Some(body) = world.get_body(0) {
        assert_eq!(body.mass, 2.0);
        assert_eq!(body.position, Vec2::new(1.0, 2.0));
    } else {
        panic!("Body not found");
    }
}

#[test]
fn test_step_simulation() {
    let mut world = World::new();
    let index = world.create_body(1.0, Vec2::new(0.0, 0.0));

    // Step the simulation for 1 second
    world.step(1.0);

    if let Some(body) = world.get_body(index) {
        // Body should have fallen due to gravity (9.81 m/s²)
        // Position should be initial + velocity * dt + 0.5 * acceleration * dt²
        // But since velocity starts at 0, position = 0.5 * 9.81 * 1² = 4.905
        assert!(body.position.y > 4.0); // Approximate check
        assert!(body.velocity.y > 9.0); // Velocity should be around 9.81
    } else {
        panic!("Body not found");
    }
}
