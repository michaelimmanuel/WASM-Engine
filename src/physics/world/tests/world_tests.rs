use crate::physics::world::World;

#[test]
fn test_world_creation() {
    let world = World::new();
    assert_eq!(world.bodies_count(), 0);
}

#[test]
fn test_create_circle() {
    let mut world = World::new();
    let index = world.create_circle(2.0, 1.0, 2.0,1.0);
    assert_eq!(index, 0);
    assert_eq!(world.bodies_count(), 1);

    // Check position using the WASM-compatible getters
    assert_eq!(world.get_body_position_x(0), 1.0);
    assert_eq!(world.get_body_position_y(0), 2.0);
}


#[test]
fn test_collides_true_for_overlapping_circles() {
    let mut world = World::new();
    let i = world.create_circle(1.0, 0.0, 0.0, 1.0);
    let j = world.create_circle(1.0, 1.0, 0.0, 1.0); // distance 1, radii sum 2
    assert!(world.collides(i, j));
}

#[test]
fn test_collides_false_for_separated_circles() {
    let mut world = World::new();
    let i = world.create_circle(1.0, 0.0, 0.0, 1.0);
    let j = world.create_circle(1.0, 3.0, 0.0, 1.0); // distance 3, radii sum 2
    assert!(!world.collides(i, j));
}
