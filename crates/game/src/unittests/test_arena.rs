use crate::arena::Arena;
use glam::Vec2;

#[test]
fn test_approximate_position() {
    let original = Vec2::new(1.16, -5.42);
    let (x, y, xs, ys) = Arena::get_approximate_position(original);
    let converted = Arena::approx_to_position(x, y, xs, ys);
    let diff = original - converted;

    assert!(diff.length() < 0.5);
}
