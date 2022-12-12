use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;

/// Create a unit vector representing the
/// given angle (in radians).
/// Starts at the top of a unit circle and goes clockwise.
/// 0      => points down
/// PI / 2 => points right
/// PI     => points up
/// 3PI/2  => points left
pub fn vec2_from_angle(angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    Vec2::new(sin, cos)
}

pub fn angle_from_vec2(vec2: Vec2) -> f32 {
    vec2.x.atan2(vec2.y)
}

pub fn draw_hitbox(ctx: &Context, canvas: &mut Canvas, hitbox: Rect) {
    canvas.draw(
        &Mesh::new_rectangle(ctx, DrawMode::stroke(1.), hitbox, Color::RED).unwrap(),
        DrawParam::default(),
    );
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use super::*;

    #[test]
    fn can_convert_angle_to_vec2() {
        assert_eq!(vec2_from_angle(0.).as_ivec2(), [0, 1].into());
        assert_eq!(vec2_from_angle(PI).as_ivec2(), [0, -1].into());
        assert_eq!(vec2_from_angle(FRAC_PI_2).as_ivec2(), [1, 0].into());
        assert_eq!(vec2_from_angle(3. * FRAC_PI_2).as_ivec2(), [-1, 0].into());
    }

    #[test]
    fn can_convert_vec2_to_angle() {
        assert_eq!(angle_from_vec2([0., 1.].into()), 0.);
        assert_eq!(angle_from_vec2([0., -1.].into()), PI);
        assert_eq!(angle_from_vec2([1., 0.].into()), PI / 2.);
        assert_eq!(angle_from_vec2([-1., 0.].into()), PI / -2.);
    }
}
