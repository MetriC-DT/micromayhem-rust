use crate::arena::Arena;
use glam::Vec2;

#[test]
fn test_approximate_position() {
    let originals = [
        Vec2::new(1.16, -5.42),
        Vec2::new(0.0, 0.0),
        Vec2::new(251.95, 312.5),
    ];

    for &original in originals.iter() {
        let (x, y, xs, ys) = Arena::get_approximate_position(original);
        let converted = Arena::approx_to_position(x, y, xs, ys);
        let diff = original - converted;

        assert!(diff.length() < 1.0);
    }
}
