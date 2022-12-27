use std::rc::Rc;

use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, Mesh, Rect};
use ggez::mint::Point2;
use ggez::Context;

use crate::map::{MAP_WIDTH, MAP_X_START};
use crate::player::CollisionAction;
use crate::util::vec2_from_angle;

pub trait Shift {
    fn shift(&mut self, direction: f32, magnitude: f32);
}

pub struct Object {
    pub position: Point2<f32>,
    pub image: Rc<Image>,
    pub collision_action: CollisionAction,
    movement: Option<fn(&mut Self)>,
}

pub struct LineObject {
    pub points: [Point2<f32>; 2],
    pub width: f32,
    pub color: Color,
}

impl LineObject {
    pub fn new(points: [Point2<f32>; 2], width: f32, color: Color) -> Self {
        Self {
            points,
            width,
            color,
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut Canvas) {
        canvas.draw(
            &Mesh::new_line(ctx, &self.points, self.width, self.color).unwrap(),
            DrawParam::default(),
        )
    }
}

impl Object {
    pub fn immovable(
        position: Point2<f32>,
        image: &Rc<Image>,
        collision_action: CollisionAction,
    ) -> Self {
        Self::new(position, image, collision_action, None)
    }

    pub fn movable(
        position: Point2<f32>,
        image: &Rc<Image>,
        collision_action: CollisionAction,
        movement: fn(&mut Self),
    ) -> Self {
        Self::new(position, image, collision_action, Some(movement))
    }

    fn new(
        position: Point2<f32>,
        image: &Rc<Image>,
        collision_action: CollisionAction,
        movement: Option<fn(&mut Self)>,
    ) -> Self {
        Self {
            position,
            image: image.clone(),
            collision_action,
            movement,
        }
    }

    pub fn apply_movement(&mut self) {
        if let Some(f) = self.movement {
            f(self)
        }
    }

    pub fn hitbox(&self) -> Rect {
        let width = self.image.width() as f32;
        let height = self.image.height() as f32;
        Rect::new(self.position.x, self.position.y + height, width, 5.)
    }
}

impl Shift for Object {
    fn shift(&mut self, direction: f32, magnitude: f32) {
        let mut pos: Vec2 = self.position.into();
        let v2 = vec2_from_angle(direction);
        pos += v2 * magnitude;
        let start = MAP_X_START as f32;
        let end = MAP_WIDTH as f32;
        if pos.x > end {
            pos.x = start;
        } else if pos.x < start {
            pos.x = end;
        }
        self.position = pos.into();
    }
}

impl Shift for LineObject {
    fn shift(&mut self, direction: f32, magnitude: f32) {
        self.points = self.points.map(|v| {
            let v2 = vec2_from_angle(direction);
            let new: Vec2 = v2 * magnitude;
            let v: Vec2 = v.into();
            (v + new).into()
        });
    }
}
