use std::rc::Rc;

use ggez::glam::Vec2;
use ggez::graphics::{Image, Rect};
use ggez::mint::Point2;

use crate::map::{MAP_WIDTH, MAP_X_START};
use crate::player::CollisionAction;
use crate::util::vec2_from_angle;

pub struct Object {
    pub position: Point2<f32>,
    pub image: Rc<Image>,
    pub collision_action: CollisionAction,
    movement: Option<fn(&mut Self)>,
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

    pub fn shift(&mut self, direction: f32, magnitude: f32) {
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

    pub fn hitbox(&self) -> Rect {
        let width = self.image.width() as f32;
        let height = self.image.height() as f32;
        Rect::new(self.position.x, self.position.y + height, width, 5.)
    }
}
