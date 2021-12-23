pub fn min_float(a: f32, b: f32) -> f32 {
    return (a + b - (a - b).abs()) / 2.0;
}

pub fn max_float(a: f32, b: f32) -> f32 {
    return (a + b + (a - b).abs()) / 2.0;
}
