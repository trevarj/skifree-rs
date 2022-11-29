use ggez::graphics::Image;
use ggez::mint::Point2;

pub struct Map {}

pub struct ImmovableObject {
    position: Point2<f32>,
}

impl Map {}

impl Default for Map {
    fn default() -> Self {
        Self {}
    }
}
