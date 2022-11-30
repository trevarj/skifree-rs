use ggez::glam::Vec2;

/// Create a unit vector representing the
/// given angle (in radians).
/// Starts at the top of a unit circle and goes clockwise.
/// 0      => 90 degrees (points up)
/// PI / 2 => 0 degrees (points right)
/// PI     => 270 degrees (points down)
/// 3PI/2  => 180 degrees (points left)
pub fn vec_from_angle(angle: f32) -> Vec2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2::new(vx, vy)
}
