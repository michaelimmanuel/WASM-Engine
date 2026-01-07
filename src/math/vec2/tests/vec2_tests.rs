use super::*;

// Test constants
#[test]
fn test_constants() {
    assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
    assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
    assert_eq!(Vec2::X, Vec2::new(1.0, 0.0));
    assert_eq!(Vec2::Y, Vec2::new(0.0, 1.0));
}

// Test new
#[test]
fn test_new() {
    let v = Vec2::new(3.5, -2.7);
    assert_eq!(v.x, 3.5);
    assert_eq!(v.y, -2.7);
}

// Test dot product
#[test]
fn test_dot_product() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    assert_eq!(a.dot(b), 11.0);
}

#[test]
fn test_dot_product_orthogonal() {
    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(0.0, 1.0);
    assert_eq!(a.dot(b), 0.0);
}

// Test cross product
#[test]
fn test_cross_product() {
    let a = Vec2::new(5.0, 7.0);
    let b = Vec2::new(13.0, 17.0);
    assert_eq!(a.cross(b), -6.0);
}

#[test]
fn test_cross_product_parallel() {
    let a = Vec2::new(2.0, 4.0);
    let b = Vec2::new(1.0, 2.0);
    assert_eq!(a.cross(b), 0.0);
}

// Test length_squared
#[test]
fn test_length_squared() {
    let v = Vec2::new(3.0, 4.0);
    assert_eq!(v.length_squared(), 25.0);
}

#[test]
fn test_length_squared_zero() {
    assert_eq!(Vec2::ZERO.length_squared(), 0.0);
}

// Test length
#[test]
fn test_length() {
    let v = Vec2::new(3.0, 4.0);
    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_length_unit_vector() {
    let v = Vec2::new(1.0, 0.0);
    assert_eq!(v.length(), 1.0);
}

// Test normalize
#[test]
fn test_normalize() {
    let v = Vec2::new(3.0, 4.0);
    let normalized = v.normalize();
    assert_eq!(normalized, Vec2::new(0.6, 0.8));
    assert!((normalized.length() - 1.0).abs() < 1e-6);
}

#[test]
fn test_normalize_zero_vector() {
    let v = Vec2::ZERO;
    assert_eq!(v.normalize(), Vec2::ZERO);
}

#[test]
fn test_normalize_unit_vector() {
    let v = Vec2::new(1.0, 0.0);
    assert_eq!(v.normalize(), v);
}

// Test distance
#[test]
fn test_distance() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(3.0, 4.0);
    assert_eq!(a.distance(b), 5.0);
}

#[test]
fn test_distance_same_point() {
    let a = Vec2::new(2.0, 3.0);
    assert_eq!(a.distance(a), 0.0);
}

// Test distance_squared
#[test]
fn test_distance_squared() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(3.0, 4.0);
    assert_eq!(a.distance_squared(b), 25.0);
}

// Test is_zero
#[test]
fn test_is_zero() {
    assert!(Vec2::ZERO.is_zero());
    assert!(Vec2::new(0.0, 0.0).is_zero());
    assert!(!Vec2::new(0.0, 0.1).is_zero());
    assert!(!Vec2::new(0.1, 0.0).is_zero());
    assert!(!Vec2::new(1.0, 1.0).is_zero());
}

// Test Add operator
#[test]
fn test_addition() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    assert_eq!(a + b, Vec2::new(4.0, 6.0));
}

#[test]
fn test_addition_with_zero() {
    let a = Vec2::new(1.0, 2.0);
    assert_eq!(a + Vec2::ZERO, a);
}

// Test AddAssign operator
#[test]
fn test_add_assign() {
    let mut a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    a += b;
    assert_eq!(a, Vec2::new(4.0, 6.0));
}

// Test Sub operator
#[test]
fn test_subtraction() {
    let a = Vec2::new(5.0, 7.0);
    let b = Vec2::new(2.0, 3.0);
    assert_eq!(a - b, Vec2::new(3.0, 4.0));
}

#[test]
fn test_subtraction_with_self() {
    let a = Vec2::new(5.0, 7.0);
    assert_eq!(a - a, Vec2::ZERO);
}

// Test SubAssign operator
#[test]
fn test_sub_assign() {
    let mut a = Vec2::new(5.0, 7.0);
    let b = Vec2::new(2.0, 3.0);
    a -= b;
    assert_eq!(a, Vec2::new(3.0, 4.0));
}

// Test Mul operator
#[test]
fn test_multiplication_scalar() {
    let v = Vec2::new(2.0, 3.0);
    assert_eq!(v * 2.0, Vec2::new(4.0, 6.0));
}

#[test]
fn test_multiplication_by_zero() {
    let v = Vec2::new(2.0, 3.0);
    assert_eq!(v * 0.0, Vec2::ZERO);
}

#[test]
fn test_multiplication_by_negative() {
    let v = Vec2::new(2.0, 3.0);
    assert_eq!(v * -1.0, Vec2::new(-2.0, -3.0));
}

// Test MulAssign operator
#[test]
fn test_mul_assign() {
    let mut v = Vec2::new(2.0, 3.0);
    v *= 2.0;
    assert_eq!(v, Vec2::new(4.0, 6.0));
}

// Test Div operator
#[test]
fn test_division_scalar() {
    let v = Vec2::new(4.0, 6.0);
    assert_eq!(v / 2.0, Vec2::new(2.0, 3.0));
}

#[test]
fn test_division_by_negative() {
    let v = Vec2::new(4.0, 6.0);
    assert_eq!(v / -2.0, Vec2::new(-2.0, -3.0));
}

// Test DivAssign operator
#[test]
fn test_div_assign() {
    let mut v = Vec2::new(4.0, 6.0);
    v /= 2.0;
    assert_eq!(v, Vec2::new(2.0, 3.0));
}

// Test Neg operator
#[test]
fn test_negation() {
    let v = Vec2::new(2.0, -3.0);
    assert_eq!(-v, Vec2::new(-2.0, 3.0));
}

#[test]
fn test_negation_zero() {
    assert_eq!(-Vec2::ZERO, Vec2::ZERO);
}

#[test]
fn test_double_negation() {
    let v = Vec2::new(2.0, 3.0);
    assert_eq!(-(-v), v);
}
