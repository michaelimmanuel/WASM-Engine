use crate::physics::world::World;

#[test]
fn test_world_creation() {
    let world = World::new();
    assert_eq!(world.bodies_count(), 0);
}

#[test]
fn test_create_body() {
    let mut world = World::new();
    let index = world.create_body(2.0, 1.0, 2.0);
    assert_eq!(index, 0);
    assert_eq!(world.bodies_count(), 1);

    // Check position using the WASM-compatible getters
    assert_eq!(world.get_body_position_x(0), 1.0);
    assert_eq!(world.get_body_position_y(0), 2.0);
}

#[test]
fn test_step_simulation() {
    let mut world = World::new();
    let index = world.create_body(1.0, 0.0, 0.0);

    // Step the simulation for 1 second
    world.step(1.0);

    // Body should have fallen due to gravity (9.81 m/s²)
    // Position should be initial + velocity * dt + 0.5 * acceleration * dt²
    // But since velocity starts at 0, position = 0.5 * 9.81 * 1² = 4.905
    assert!(world.get_body_position_y(index) > 4.0); // Approximate check
    assert!(world.get_body_position_x(index) == 0.0); // X position should remain 0
}

#[test]
fn test_collides_true_for_overlapping_circles() {
    let mut world = World::new();
    let i = world.create_body(1.0, 0.0, 0.0);
    let j = world.create_body(1.0, 1.0, 0.0); // distance 1, radii sum 2
    assert!(world.collides(i, j));
}

#[test]
fn test_collides_false_for_separated_circles() {
    let mut world = World::new();
    let i = world.create_body(1.0, 0.0, 0.0);
    let j = world.create_body(1.0, 3.0, 0.0); // distance 3, radii sum 2
    assert!(!world.collides(i, j));
}
