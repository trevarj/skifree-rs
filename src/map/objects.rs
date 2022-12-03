use std::rc::Rc;

use ggez::glam::Vec2;
use ggez::graphics::Image;
use ggez::mint::Point2;

use crate::player::CollisionAction;
use crate::util::vec_from_angle;

pub struct Object {
    pub position: Point2<f32>,
    pub image: Rc<Image>,
    /// What happens when a player hits this object
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

    pub fn shift(&mut self, direction: f32) {
        let mut pos: Vec2 = self.position.into();
        let v2 = vec_from_angle(direction);
        pos += v2 * 3.; // TODO: change speed
        self.position = pos.into();
    }
}
