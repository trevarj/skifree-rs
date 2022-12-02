use ggez::glam::Vec2;

/// Create a unit vector representing the
/// given angle (in radians).
/// Starts at the top of a unit circle and goes clockwise.
/// 0      => points down
/// PI / 2 => points right
/// PI     => points up
/// 3PI/2  => points left
pub fn vec_from_angle(angle: f32) -> Vec2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2::new(vx, vy)
}
