use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::Context;

#[derive(Default, Clone, Copy)]
pub struct Hud {
    pub distance: f32,
    pub elapsed_time: f32,
    pub score: i32,
}

impl Hud {
    pub fn set_distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    pub fn add_time(mut self, time: f32) -> Self {
        self.elapsed_time += time;
        self
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        canvas.draw(
            &Text::new(format!("distance: {}", self.distance as u32)),
            DrawParam::new().dest([0., 0.]).color(Color::BLACK),
        );
        canvas.draw(
            &Text::new(format!("elapsed time: {:.2}s", self.elapsed_time)),
            DrawParam::new().dest([0., 12.]).color(Color::BLACK),
        );
    }
}
